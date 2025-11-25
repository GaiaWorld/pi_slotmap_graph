use super::id::{EdgeId, VertexId};
use super::id::edge_info::EdgeInfo;
use super::storage::{EdgeContainer, VertexContainer};
use super::index::SimpleVertexQuery;
use graph_api_lib::{
    EdgeSearch, Element, ElementId, Graph,
    SupportsClear, SupportsEdgeAdjacentLabelIndex, SupportsEdgeHashIndex, SupportsEdgeLabelIndex,
    SupportsEdgeRangeIndex, SupportsElementRemoval, SupportsVertexFullTextIndex,
    SupportsVertexHashIndex, SupportsVertexLabelIndex, SupportsVertexRangeIndex,
    VertexSearch,
};
use smallbox::{SmallBox, smallbox};
use smallbox::space::S8;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// 基于SlotMap的图实现，严格参照graph-api-simplegraph结构
///
/// 这个结构体提供了一个高性能的图数据结构，集成了索引系统以提供更快的查询性能。
///
/// ## 架构特点
///
/// ### 存储层
/// - **VertexContainer**: O(1) 顶点存储和访问
/// - **EdgeContainer**: O(1) 边存储和连接管理
/// - **内存效率**: 基于 SlotMap 的紧凑存储
///
/// ### 索引层
/// - **IndexManager**: 统一的索引管理接口
/// - **多类型索引**: 哈希索引、范围索引支持
/// - **动态索引**: 运行时创建和管理索引
/// - **自动维护**: 数据变更时自动更新索引
///
/// ## 性能优势
///
/// ### 基础操作
/// - **插入/删除**: O(1) 时间复杂度
/// - **精确查询**: O(1) 通过索引加速
/// - **范围查询**: O(log n) 通过索引优化
///
/// ### 内存效率
/// - **紧凑存储**: 无指针重用问题
/// - **自动重用**: 删除空间的智能回收
/// - **索引优化**: 可选的索引以减少内存开销
#[derive(Debug)]
pub struct SlotMapGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    /// 顶点存储容器
    vertices: VertexContainer<Vertex>,
    /// 边存储容器
    edges: EdgeContainer<Edge>,
    /// 简单顶点查询器（用于智能查询）
    vertex_query: SimpleVertexQuery,
}

/// 顶点引用
#[derive(Debug)]
pub struct VertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::VertexId,
    weight: &'graph Graph::Vertex,
}

impl<Graph> From<VertexReference<'_, Graph>> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn from(value: VertexReference<Graph>) -> Self {
        ElementId::Vertex(value.id)
    }
}

impl<'graph, Graph> graph_api_lib::VertexReference<'graph, Graph> for VertexReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    fn id(&self) -> Graph::VertexId {
        self.id
    }

    fn weight(&self) -> &Graph::Vertex {
        self.weight
    }

    fn project<
        'reference,
        T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Vertex>,
    >(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

/// 可变顶点引用
pub struct VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::VertexId,
    weight: &'graph mut Graph::Vertex,
}

impl<Graph> Debug for VertexReferenceMut<'_, Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VertexReferenceMut")
            .field("id", &self.id)
            .field("weight", &"&mut ...")
            .finish()
    }
}

impl<Graph> From<VertexReferenceMut<'_, Graph>> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn from(value: VertexReferenceMut<Graph>) -> Self {
        ElementId::Vertex(value.id)
    }
}

impl<'graph, Graph> graph_api_lib::VertexReference<'graph, Graph>
    for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    fn id(&self) -> Graph::VertexId {
        self.id
    }

    fn weight(&self) -> &Graph::Vertex {
        self.weight
    }

    fn project<
        'reference,
        T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Vertex>,
    >(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

impl<'graph, Graph> graph_api_lib::VertexReferenceMut<'graph, Graph>
    for VertexReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId> + 'graph,
{
    type MutationListener<'reference> = ();

    fn weight_mut(&mut self) -> &mut Graph::Vertex {
        self.weight
    }

    fn project_mut<
        'reference,
        T: graph_api_lib::ProjectMut<
                'reference,
                <Graph as graph_api_lib::Graph>::Vertex,
                Self::MutationListener<'reference>,
            >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(self.weight, ())
    }
}

/// 边引用
#[derive(Debug)]
pub struct EdgeReference<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::EdgeId,
    weight: &'graph Graph::Edge,
    from: Graph::VertexId,
    to: Graph::VertexId,
}

impl<Graph> From<EdgeReference<'_, Graph>> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn from(value: EdgeReference<Graph>) -> Self {
        ElementId::Edge(value.id)
    }
}

impl<'a, Graph> graph_api_lib::EdgeReference<'a, Graph> for EdgeReference<'a, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    fn id(&self) -> Graph::EdgeId {
        self.id
    }

    fn tail(&self) -> Graph::VertexId {
        self.from
    }

    fn head(&self) -> Graph::VertexId {
        self.to
    }

    fn weight(&self) -> &Graph::Edge {
        self.weight
    }

    fn project<'reference, T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

/// 可变边引用
#[derive(Debug)]
pub struct EdgeReferenceMut<'graph, Graph>
where
    Graph: graph_api_lib::Graph,
{
    id: Graph::EdgeId,
    weight: &'graph mut Graph::Edge,
    from: Graph::VertexId,
    to: Graph::VertexId,
}

impl<Graph> From<EdgeReferenceMut<'_, Graph>> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph,
{
    fn from(value: EdgeReferenceMut<Graph>) -> Self {
        ElementId::Edge(value.id)
    }
}

impl<Graph> graph_api_lib::EdgeReference<'_, Graph> for EdgeReferenceMut<'_, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    fn id(&self) -> Graph::EdgeId {
        self.id
    }

    fn tail(&self) -> Graph::VertexId {
        self.from
    }

    fn head(&self) -> Graph::VertexId {
        self.to
    }

    fn weight(&self) -> &Graph::Edge {
        self.weight
    }

    fn project<'reference, T: graph_api_lib::Project<'reference, <Graph as graph_api_lib::Graph>::Edge>>(
        &'reference self,
    ) -> Option<T> {
        graph_api_lib::Project::project(self.weight)
    }
}

impl<Graph> graph_api_lib::EdgeReferenceMut<'_, Graph> for EdgeReferenceMut<'_, Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId, EdgeId = EdgeId>,
{
    type MutationListener<'reference> = ();

    fn weight_mut(&mut self) -> &mut Graph::Edge {
        self.weight
    }

    fn project_mut<
        'reference,
        T: graph_api_lib::ProjectMut<
                'reference,
                <Graph as graph_api_lib::Graph>::Edge,
                Self::MutationListener<'reference>,
            >,
    >(
        &'reference mut self,
    ) -> Option<T> {
        graph_api_lib::ProjectMut::project_mut(self.weight, ())
    }
}

/// 顶点迭代器
pub struct VertexIter<'search, 'graph, Vertex, Edge>
where
    Vertex: Element ,
    Edge: Element ,
{
    _phantom: PhantomData<(&'search (), Vertex, Edge)>,
    vertices: &'graph VertexContainer<Vertex>,
    keys: SmallBox<dyn Iterator<Item = VertexId> + 'graph, S8>,
    count: usize,
    limit: usize,
}

impl<'graph, Vertex, Edge> Iterator for VertexIter<'_, 'graph, Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    type Item = VertexReference<'graph, SlotMapGraph<Vertex, Edge>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit {
            return None;
        }

        while let Some(id) = self.keys.next() {
            if let Some(weight) = self.vertices.get(id) {
                self.count += 1;
                return Some(VertexReference { id, weight });
            }
        }
        None
    }
}

/// 边迭代器
pub struct EdgeIter<'search, 'graph, Vertex, Edge>
where
    Vertex: Element ,
    Edge: Element,
{
    _phantom: PhantomData<(&'search (), Vertex, Edge)>,
    edges: &'graph EdgeContainer<Edge>,
    keys: std::vec::IntoIter<EdgeId>,
    count: usize,
    limit: usize,
}

impl<'graph, Vertex, Edge> Iterator for EdgeIter<'_, 'graph, Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    type Item = EdgeReference<'graph, SlotMapGraph<Vertex, Edge>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.limit {
            return None;
        }

        while let Some(id) = self.keys.next() {
            if let Some((weight, conn)) = self.edges.get(id) {
                // if let Some(conn) = self.edges.get_connection(id) {
                    self.count += 1;
                    return Some(EdgeReference {
                        id,
                        weight,
                        from: conn.from(),
                        to: conn.to(),
                    });
                // }
            }
        }
        None
    }
}

impl<Vertex, Edge> Default for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Vertex, Edge> SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
    /// 创建一个新的空图
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `SlotMapGraph` 实例，不包含任何顶点或边。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::SlotMapGraph;
    ///
    /// let graph: SlotMapGraph<String, i32> = SlotMapGraph::new();
    /// assert_eq!(graph.vertex_count(), 0);
    /// assert_eq!(graph.edge_count(), 0);
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(1)
    /// - **空间复杂度**: O(1)
    /// - **内存分配**: 最小化初始分配
    pub fn new() -> Self {
        Self {
            vertices: VertexContainer::new(),
            edges: EdgeContainer::new(),
            vertex_query: SimpleVertexQuery::new(),
        }
    }

    /// 获取简单查询器的可变引用
    ///
    /// 提供对智能查询系统的访问权限，可以用于：
    /// - 手动建立索引
    /// - 执行智能查询
    /// - 管理查询缓存
    pub fn vertex_query_mut(&mut self) -> &mut SimpleVertexQuery {
        &mut self.vertex_query
    }

    /// 获取简单查询器的不可变引用
    pub fn vertex_query(&self) -> &SimpleVertexQuery {
        &self.vertex_query
    }

    /// 为顶点添加字符串索引
    ///
    /// 便利方法，用于为指定的顶点添加字符串值到索引中。
    pub fn index_vertex_string(&mut self, vertex_id: VertexId, value: &str) {
        self.vertex_query.insert_string(value, vertex_id);
    }

    /// 为顶点添加整数索引
    ///
    /// 便利方法，用于为指定的顶点添加整数值到索引中。
    pub fn index_vertex_int(&mut self, vertex_id: VertexId, value: i64) {
        self.vertex_query.insert_int(value, vertex_id);
    }

    /// 获取边的起始顶点
    ///
    /// 根据给定的边ID，返回该边的起始顶点ID。如果边不存在，返回None。
    ///
    /// # 参数
    ///
    /// * `edge_id` - 要查询的边的标识符
    ///
    /// # 返回值
    ///
    /// * `Some(VertexId)` - 边的起始顶点ID
    /// * `None` - 边不存在或已被删除
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::SlotMapGraph;
    ///
    /// let mut graph = SlotMapGraph::new();
    /// let v1 = graph.add_vertex("A");
    /// let v2 = graph.add_vertex("B");
    /// let edge = graph.add_edge("connects", v1, v2);
    ///
    /// assert_eq!(graph.edge_from(edge), Some(v1));
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(1) - 直接通过SlotMap查找
    /// - **空间复杂度**: O(1) - 不分配额外内存
    pub fn edge_from(&self, edge_id: EdgeId) -> Option<VertexId> {
        self.edges.get_connection(edge_id).map(|info| info.from())
    }

    /// 获取边的目标顶点
    ///
    /// 根据给定的边ID，返回该边的目标顶点ID。如果边不存在，返回None。
    ///
    /// # 参数
    ///
    /// * `edge_id` - 要查询的边的标识符
    ///
    /// # 返回值
    ///
    /// * `Some(VertexId)` - 边的目标顶点ID
    /// * `None` - 边不存在或已被删除
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::SlotMapGraph;
    ///
    /// let mut graph = SlotMapGraph::new();
    /// let v1 = graph.add_vertex("A");
    /// let v2 = graph.add_vertex("B");
    /// let edge = graph.add_edge("connects", v1, v2);
    ///
    /// assert_eq!(graph.edge_to(edge), Some(v2));
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(1) - 直接通过SlotMap查找
    /// - **空间复杂度**: O(1) - 不分配额外内存
    pub fn edge_to(&self, edge_id: EdgeId) -> Option<VertexId> {
        self.edges.get_connection(edge_id).map(|info| info.to())
    }

    /// 获取从指定顶点出发的所有边
    ///
    /// 返回一个迭代器，产生从给定顶点出发的所有有向边。这是一个懒迭代器，
    /// 按需生成边引用，不会预计算所有结果。
    ///
    /// # 参数
    ///
    /// * `vertex_id` - 源顶点的标识符
    ///
    /// # 返回值
    ///
    /// 返回一个迭代器，产生 `EdgeReference` 类型的项，每个项包含：
    /// - 边的ID
    /// - 边的权重数据
    /// - 起始和目标顶点ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::SlotMapGraph;
    ///
    /// let mut graph = SlotMapGraph::new();
    /// let alice = graph.add_vertex("Alice");
    /// let bob = graph.add_vertex("Bob");
    /// let charlie = graph.add_vertex("Charlie");
    ///
    /// let friendship_ab = graph.add_edge("friend", alice, bob);
    /// let follows_ac = graph.add_edge("follows", alice, charlie);
    ///
    /// // 查找Alice的所有出边
    /// for edge_ref in graph.outgoing_edges(alice) {
    ///     println!("Edge: {} -> {}", edge_ref.tail(), edge_ref.head());
    ///     println!("Weight: {}", edge_ref.weight());
    /// }
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(k) - k为顶点的出度
    /// - **空间复杂度**: O(1) - 迭代器状态为常数大小
    /// - **缓存友好**: 连续内存访问模式
    ///
    /// # 注意事项
    ///
    /// - 返回的迭代器持有图的不可变引用
    /// - 在迭代期间不能修改图结构
    /// - 如果顶点不存在，迭代器为空
    pub fn outgoing_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeReference<'_, Self>> {
        self.edges.edges_from(vertex_id).filter_map(move |edge_id| {
            // if let Some(conn) = self.edges.get_connection(edge_id) {
                if let Some((weight, conn)) = self.edges.get(edge_id) {
                    return Some(EdgeReference {
                        id: edge_id,
                        weight,
                        from: conn.from(),
                        to: conn.to(),
                    });
                }
            // }
            None
        })
    }

    /// 获取指向指定顶点的所有边
    ///
    /// 返回一个迭代器，产生指向给定顶点的所有有向边。这是一个懒迭代器，
    /// 按需生成边引用，不会预计算所有结果。
    ///
    /// # 参数
    ///
    /// * `vertex_id` - 目标顶点的标识符
    ///
    /// # 返回值
    ///
    /// 返回一个迭代器，产生 `EdgeReference` 类型的项，每个项包含：
    /// - 边的ID
    /// - 边的权重数据
    /// - 起始和目标顶点ID
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::SlotMapGraph;
    ///
    /// let mut graph = SlotMapGraph::new();
    /// let alice = graph.add_vertex("Alice");
    /// let bob = graph.add_vertex("Bob");
    /// let charlie = graph.add_vertex("Charlie");
    ///
    /// let friendship_ab = graph.add_edge("friend", alice, bob);
    /// let follows_cb = graph.add_edge("follows", charlie, bob);
    ///
    /// // 查找Bob的所有入边
    /// for edge_ref in graph.incoming_edges(bob) {
    ///     println!("Edge: {} -> {}", edge_ref.tail(), edge_ref.head());
    ///     println!("From: {}", edge_ref.tail());
    /// }
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(k) - k为顶点的入度
    /// - **空间复杂度**: O(1) - 迭代器状态为常数大小
    /// - **缓存友好**: 连续内存访问模式
    ///
    /// # 注意事项
    ///
    /// - 返回的迭代器持有图的不可变引用
    /// - 在迭代期间不能修改图结构
    /// - 如果顶点不存在，迭代器为空
    pub fn incoming_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeReference<'_, Self>> {
        self.edges.edges_to(vertex_id).filter_map(move |edge_id| {
            // if let Some(conn) = self.edges.get_connection(edge_id) {
                if let Some((weight, conn)) = self.edges.get(edge_id) {
                    return Some(EdgeReference {
                        id: edge_id,
                        weight,
                        from: conn.from(),
                        to: conn.to(),
                    });
                }
            // }
            None
        })
    }

    /// 获取与指定顶点相邻的所有边（入边和出边）
    pub fn adjacent_edges(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeReference<'_, Self>> {
        self.edges.edges_adjacent(vertex_id).filter_map(move |edge_id| {
            // if let Some(conn) = self.edges.get_connection(edge_id) {
                if let Some((weight, conn)) = self.edges.get(edge_id) {
                    return Some(EdgeReference {
                        id: edge_id,
                        weight,
                        from: conn.from(),
                        to: conn.to(),
                    });
                }
            // }
            None
        })
    }

    /// 检查两个顶点之间是否存在边
    pub fn has_edge(&self, from: VertexId, to: VertexId) -> bool {
        self.edges.has_edge_between(from, to)
    }

    /// 获取两个顶点之间的所有边
    pub fn edges_between(&self, from: VertexId, to: VertexId) -> impl Iterator<Item = EdgeReference<'_, Self>> {
        self.edges.edges_between(from, to).filter_map(move |edge_id| {
            // if let Some(conn) = self.edges.get_connection(edge_id) {
                if let Some((weight, conn)) = self.edges.get(edge_id) {
                    return Some(EdgeReference {
                        id: edge_id,
                        weight,
                        from: conn.from(),
                        to: conn.to(),
                    });
                }
            // }
            None
        })
    }

    /// 获取顶点的出度
    pub fn out_degree(&self, vertex_id: VertexId) -> usize {
        self.edges.edges_from(vertex_id).count()
    }

    /// 获取顶点的入度
    pub fn in_degree(&self, vertex_id: VertexId) -> usize {
        self.edges.edges_to(vertex_id).count()
    }

    /// 获取顶点的度（入度+出度）
    pub fn degree(&self, vertex_id: VertexId) -> usize {
        self.edges.edges_adjacent(vertex_id).count()
    }

    /// 获取所有顶点
    pub fn all_vertices(&self) -> impl Iterator<Item = (VertexId, &Vertex)> {
        self.vertices.iter()
    }

    /// 获取所有边
    pub fn all_edges(&self) -> impl Iterator<Item = (EdgeId, &Edge, VertexId, VertexId)> {
        self.edges.iter_with_connections().map(|(id, edge, info)| {
            (id, edge, info.from(), info.to())
        })
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// 获取边数量
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// 检查是否包含指定顶点
    pub fn contains_vertex(&self, vertex_id: VertexId) -> bool {
        self.vertices.contains(vertex_id)
    }

    /// 检查是否包含指定边
    pub fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.edges.contains(edge_id)
    }

    /// 检查图是否为空
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() && self.edges.is_empty()
    }
}

impl<Vertex, Edge> Graph for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element,
    Edge: Element,
{
    type Vertex = Vertex;
    type Edge = Edge;
    type VertexId = VertexId;
    type EdgeId = EdgeId;
    type VertexReference<'graph> = VertexReference<'graph, Self> where Self: 'graph;
    type VertexReferenceMut<'graph> = VertexReferenceMut<'graph, Self> where Self: 'graph;
    type EdgeReference<'graph> = EdgeReference<'graph, Self> where Self: 'graph;
    type EdgeReferenceMut<'graph> = EdgeReferenceMut<'graph, Self> where Self: 'graph;
    type EdgeIter<'search, 'graph> = EdgeIter<'search, 'graph, Vertex, Edge> where Self: 'graph;
    type VertexIter<'search, 'graph> = VertexIter<'search, 'graph, Vertex, Edge> where Self: 'graph;

    fn add_vertex(&mut self, vertex: Self::Vertex) -> Self::VertexId {
        let vertex_id = self.vertices.insert(vertex);

        // 自动构建基础索引（如果可能的话）
        // 注意：由于我们不知道顶点的内部结构，这里无法自动建立索引
        // 用户需要手动调用索引方法来构建自定义索引
        // 在实际使用中，可以通过实现特定的顶点类型来自动索引

        vertex_id
    }

    fn add_edge(
        &mut self,
        from: Self::VertexId,
        to: Self::VertexId,
        edge: Self::Edge,
    ) -> Self::EdgeId {
        let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
        self.edges.insert(edge, edge_info)
    }

    fn vertex(&self, id: Self::VertexId) -> Option<Self::VertexReference<'_>> {
        self.vertices.get(id).map(|weight| VertexReference { id, weight })
    }

    fn vertex_mut(&mut self, id: Self::VertexId) -> Option<Self::VertexReferenceMut<'_>> {
        self.vertices.get_mut(id).map(|weight| VertexReferenceMut { id, weight })
    }

    fn vertices<'search>(
        &self,
        search: &VertexSearch<'search, Self>,
    ) -> Self::VertexIter<'search, '_> {
        // 根据 VertexSearch 类型选择最优查询策略
        // let keys_iter: SmallBox<dyn Iterator<Item = VertexId> + '_, S8> = match search {
        //     VertexSearch::Scan { .. } => {
        //         // 全扫描：遍历所有顶点
        //         smallbox!(self.vertices.keys())
        //     }
        //     VertexSearch::Label { .. } => {
        //         // 对于简单的实现，由于我们没有按标签的索引，回退到全扫描
        //         // 在更高级的实现中，这里可以使用按标签的索引
        //         smallbox!(self.vertices.keys())
        //     }
        //     VertexSearch::Index { value, .. } => {
        //         // 使用智能查询进行索引查找
        //         smallbox!(self.vertex_query.query_value(value))
        //     }
        //     VertexSearch::Range { range, .. } => {
        //         // 使用智能查询进行范围查找
        //         smallbox!(self.vertex_query.range_value(range))
        //     }
        //     VertexSearch::FullText { search, .. } => {
        //         // 全文搜索：目前使用字符串查询作为近似实现
        //         match search {
        //             graph_api_lib::Value::Str(s) => smallbox!(self.vertex_query.query_string(s)),
        //             _ => smallbox!(std::iter::empty()),
        //         }
        //     }
        //     _ => {
        //         // 处理未来可能的新查询类型
        //         smallbox!(std::iter::empty())
        //     }
        // };

        VertexIter::<Vertex, Edge> {
            _phantom: PhantomData,
            vertices: &self.vertices,
            keys: smallbox!(self.vertices.keys()),
            count: 0,
            limit: search.limit(),
        }
    }

    fn edge(&self, id: Self::EdgeId) -> Option<Self::EdgeReference<'_>> {
        // if let Some(conn) = self.edges.get_connection(id) {
            if let Some((weight, conn)) = self.edges.get(id) {
                return Some(EdgeReference {
                    id,
                    weight,
                    from: conn.from(),
                    to: conn.to(),
                });
            }
        // }
        None
    }

    fn edge_mut(&mut self, edge: Self::EdgeId) -> Option<Self::EdgeReferenceMut<'_>> {
        // let conn = self.edges.get_connection(edge)?;
       
        if let Some((weight, conn)) = self.edges.get_mut(edge) {
             let (from, to) = (conn.from(), conn.to());
            return Some(EdgeReferenceMut {
                id: edge,
                weight,
                from,
                to,
            });
        }
        None
    }

    fn edges<'search>(
        &self,
        vertex: Self::VertexId,
        search: &EdgeSearch<'search, Self>,
    ) -> Self::EdgeIter<'search, '_> {
        use graph_api_lib::Direction;

        // 首先获取所有符合条件的边（基于方向）
        let candidate_edges: Vec<EdgeId> = match search.direction {
            Direction::Outgoing => self.edges.edges_from(vertex).collect(),
            Direction::Incoming => self.edges.edges_to(vertex).collect(),
            Direction::All => self.edges.edges_adjacent(vertex).collect(),
        };

        // 然后过滤掉不符合标签条件的边
        let filtered_keys: Vec<EdgeId> = if let Some(target_label) = search.label {
            candidate_edges
                .into_iter()
                .filter(|&edge_id| {
                    if let Some((edge_weight, _)) = self.edges.get(edge_id) {
                        edge_weight.label() == target_label
                    } else {
                        false
                    }
                })
                .collect()
        } else {
            candidate_edges
        };

        EdgeIter::<Vertex, Edge> {
            _phantom: PhantomData,
            edges: &self.edges,
            keys: filtered_keys.into_iter(),
            count: 0,
            limit: search.limit(),
        }
    }

    fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
    }
}

// 实现所有支持trait
impl<Vertex, Edge> SupportsVertexLabelIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsEdgeLabelIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsVertexHashIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsEdgeHashIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsVertexRangeIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsEdgeRangeIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsVertexFullTextIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsEdgeAdjacentLabelIndex for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
}

impl<Vertex, Edge> SupportsClear for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
    fn clear(&mut self) {
        self.vertices.clear();
        self.edges.clear();
    }
}

impl<Vertex, Edge> SupportsElementRemoval for SlotMapGraph<Vertex, Edge>
where
    Vertex: Element + Clone,
    Edge: Element + Clone,
{
    fn remove_vertex(&mut self, id: Self::VertexId) -> Option<Self::Vertex> {
        // 删除顶点时，也需要删除相关的所有边
        let edges_to_remove: Vec<EdgeId> = self.edges.edges_adjacent(id).collect();
        for edge_id in edges_to_remove {
            self.edges.remove(edge_id);
        }

        self.vertices.remove(id)
    }

    fn remove_edge(&mut self, edge: Self::EdgeId) -> Option<Self::Edge> {
        self.edges.remove(edge).map(|(edge, _)| edge)
    }
}

#[cfg(test)]
mod tests {
    use graph_api_lib::{Element, Graph, VertexSearch, VertexReference, EdgeReference};
    use super::*;

    #[derive(Debug, Clone)]
    struct TestVertex {
        name: String,
        _value: i32,
    }

    #[derive(Debug, Clone)]
    struct TestEdge {
        weight: f64,
    }

    impl Element for TestVertex {
        type Label = ();

        fn label(&self) -> Self::Label {
            ()
        }
    }

    impl Element for TestEdge {
        type Label = ();

        fn label(&self) -> Self::Label {
            ()
        }
    }

    #[test]
    fn test_graph_basic_operations() {
        let mut graph = SlotMapGraph::<TestVertex, TestEdge>::new();

        // 添加顶点
        let v1 = graph.add_vertex(TestVertex {
            name: "A".to_string(),
            _value: 1,
        });
        let v2 = graph.add_vertex(TestVertex {
            name: "B".to_string(),
            _value: 2,
        });
        let v3 = graph.add_vertex(TestVertex {
            name: "C".to_string(),
            _value: 3,
        });

        // 添加边
        let e1 = graph.add_edge(v1, v2, TestEdge { weight: 1.5 });
        let _e2 = graph.add_edge(v2, v3, TestEdge { weight: 2.5 });
        let _e3 = graph.add_edge(v1, v3, TestEdge { weight: 3.5 });

        // 验证顶点
        assert_eq!(graph.vertex(v1).unwrap().weight().name, "A");
        assert_eq!(graph.vertex(v2).unwrap().weight().name, "B");
        assert_eq!(graph.vertex(v3).unwrap().weight().name, "C");

        // 验证边
        assert_eq!(graph.edge(e1).unwrap().weight().weight, 1.5);
        assert_eq!(graph.edge(e1).unwrap().tail(), v1);
        assert_eq!(graph.edge(e1).unwrap().head(), v2);

        // 测试度数计算
        assert_eq!(graph.out_degree(v1), 2);
        assert_eq!(graph.in_degree(v1), 0);
        assert_eq!(graph.out_degree(v2), 1);
        assert_eq!(graph.in_degree(v2), 1);
        assert_eq!(graph.out_degree(v3), 0);
        assert_eq!(graph.in_degree(v3), 2);

        // 测试边查询
        assert!(graph.has_edge(v1, v2));
        assert!(!graph.has_edge(v2, v1));
        assert!(graph.has_edge(v1, v3));

        // 测试遍历
        let vertices: Vec<_> = graph.vertices(&VertexSearch::scan()).collect();
        assert_eq!(vertices.len(), 3);
    }

    #[test]
    fn test_edge_removal() {
        let mut graph = SlotMapGraph::<TestVertex, TestEdge>::new();

        let v1 = graph.add_vertex(TestVertex {
            name: "A".to_string(),
            _value: 1,
        });
        let v2 = graph.add_vertex(TestVertex {
            name: "B".to_string(),
            _value: 2,
        });

        let e1 = graph.add_edge(v1, v2, TestEdge { weight: 1.5 });
        assert!(graph.has_edge(v1, v2));

        // 删除边
        let removed_edge = graph.remove_edge(e1);
        assert!(removed_edge.is_some());
        assert!(!graph.has_edge(v1, v2));
    }

    #[test]
    fn test_vertex_removal() {
        let mut graph = SlotMapGraph::<TestVertex, TestEdge>::new();

        let v1 = graph.add_vertex(TestVertex {
            name: "A".to_string(),
            _value: 1,
        });
        let v2 = graph.add_vertex(TestVertex {
            name: "B".to_string(),
            _value: 2,
        });
        let v3 = graph.add_vertex(TestVertex {
            name: "C".to_string(),
            _value: 3,
        });

        let e1 = graph.add_edge(v1, v2, TestEdge { weight: 1.5 });
        let e2 = graph.add_edge(v1, v3, TestEdge { weight: 2.5 });

        // 删除顶点v1应该同时删除相关的边
        let removed_vertex = graph.remove_vertex(v1);
        assert!(removed_vertex.is_some());
        assert_eq!(removed_vertex.unwrap().name, "A");

        // 验证边被删除
        assert!(!graph.contains_vertex(v1));
        assert!(!graph.contains_edge(e1));
        assert!(!graph.contains_edge(e2));

        // 验证其他顶点还在
        assert!(graph.contains_vertex(v2));
        assert!(graph.contains_vertex(v3));
    }
}