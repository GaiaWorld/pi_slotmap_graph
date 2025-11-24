/**
 * # EdgeContainer - 边存储容器
 *
 * 基于 `pi_slotmap::SlotMap` 的高性能边存储实现。
 *
 * ## 设计原则
 *
 * - **高性能**: O(1) 插入、删除、查找操作
 * - **内存效率**: 紧凑存储，自动重用删除的空间
 * - **类型安全**: 强类型的泛型设计
 * - **连接管理**: 内置连接信息管理
 *
 * ## 核心特性
 *
 * ### 存储性能
 * - **插入**: O(1) 平均时间复杂度
 * - **删除**: O(1) 平均时间复杂度
 * - **查找**: O(1) 时间复杂度
 * - **内存**: 紧凑存储，无额外开销
 */

use super::{Storage, StorageKey};
use super::super::id::{EdgeId, EdgeInfo, VertexId};
use graph_api_lib::Element;
use pi_slotmap::{DefaultKey, SlotMap};

/// 边存储容器，基于 `pi_slotmap::SlotMap` 实现
#[derive(Debug)]
pub struct EdgeContainer<E>
where
    E: Element,
{
    /// 使用 SlotMap 存储边数据
    data: SlotMap<DefaultKey, (E, EdgeInfo)>,
}

impl<E> EdgeContainer<E>
where
    E: Element,
{
    /// 创建新的空边容器
    #[inline]
    pub fn new() -> Self {
        Self {
            data: SlotMap::new(),
            // connections: SlotMap::new(),
        }
    }

    /// 创建带有预设容量的边容器
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: SlotMap::with_capacity(capacity),
            // connections: SlotMap::with_capacity(capacity),
        }
    }

    /// 插入边数据和连接信息
    #[inline]
    pub fn insert(&mut self, edge: E, edge_info: EdgeInfo) -> EdgeId {
        let key = self.data.insert((edge, edge_info));

        EdgeId::new(key)
    }

    /// 获取边数据的不可变引用
    #[inline]
    pub fn get(&self, id: EdgeId) -> Option<&E> {
        self.data.get(id.key()).map(|v|&v.0)
    }

    /// 获取边数据的可变引用
    #[inline]
    pub fn get_mut(&mut self, id: EdgeId) -> Option<&mut E> {
        self.data.get_mut(id.key()).map(|v|&mut v.0)
    }

    /// 获取连接信息的不可变引用
    #[inline]
    pub fn get_connection(&self, id: EdgeId) -> Option<&EdgeInfo> {
        self.data.get(id.key()).map(|v|&v.1)
    }

    /// 删除边
    #[inline]
    pub fn remove(&mut self, id: EdgeId) -> Option<(E, EdgeInfo)> {
        let edge = self.data.remove(id.key());

        match edge {
            Some((e, i)) => Some((e, i)),
            _ => None,
        }
    }

    /// 检查是否包含指定边
    #[inline]
    pub fn contains(&self, id: EdgeId) -> bool {
        self.data.contains_key(id.key())
    }

    /// 获取边数量
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查容器是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有边
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        // self.connections.clear();
    }

    /// 获取所有边ID
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.data.keys().map(EdgeId::new)
    }

    // /// 获取所有边数据的不可变引用
    // #[inline]
    // pub fn values(&self) -> impl Iterator<Item = &E> + '_ {
    //     self.data.values()
    // }

    // /// 获取所有边数据的可变引用
    // #[inline]
    // pub fn values_mut(&mut self) -> impl Iterator<Item = &mut E> + '_ {
    //     self.data.values_mut()
    // }

    /// 迭代所有边的键值对
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (EdgeId, &E)> + '_ {
        self.data.iter().map(|(key, (value, _))| (EdgeId::new(key), value))
    }

    /// 迭代所有边的键值对（可变）
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EdgeId, &mut E)> + '_ {
        self.data.iter_mut().map(|(key, (value, _))| (EdgeId::new(key), value))
    }

    /// 迭代所有边和连接信息
    #[inline]
    pub fn iter_with_connections(&self) -> impl Iterator<Item = (EdgeId, &E, &EdgeInfo)> + '_ {
        self.data.iter()
            .map(|(key, (edge, info))| (EdgeId::new(key), edge, info))
    }

    /// 保留满足条件的边
    ///
    /// 这个方法实现了两阶段删除策略，以避免在迭代过程中修改集合的问题。
    /// 首先收集所有需要删除的键，然后统一删除，确保迭代器的安全性。
    ///
    /// # 参数
    ///
    /// * `predicate` - 判断函数，返回true保留边，false删除边
    ///
    /// # 实现细节
    ///
    /// 1. **收集阶段**: 遍历所有边，收集不满足条件的键
    /// 2. **删除阶段**: 批量删除收集到的键
    ///
    /// 这种两阶段策略避免了在迭代过程中修改集合导致的未定义行为。
    #[inline]
    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(EdgeId, &E, &EdgeInfo) -> bool,
    {
        // 第一阶段：收集需要删除的键
        // 使用单独的Vec来存储键，避免在迭代过程中修改SlotMap
        let mut keys_to_remove = Vec::new();

        // 遍历所有边和连接信息
        // 注意：这里不能在迭代过程中直接删除元素，因为会违反借用检查器规则
        for ((key ), (edge, info)) in self.data.iter() {
            let edge_id = EdgeId::new(key);
            // 如果谓词返回false，标记该边为待删除
            if !predicate(edge_id, edge, info) {
                keys_to_remove.push(key);
            }
        }

        // 第二阶段：批量删除收集到的键
        // 现在可以安全地删除元素，因为迭代已经完成
        for key in keys_to_remove {
            self.data.remove(key);
        }
    }

    /// 批量插入边
    #[inline]
    pub fn insert_iter<I>(&mut self, iter: I) -> Vec<EdgeId>
    where
        I: IntoIterator<Item = (E, EdgeInfo)>,
    {
        iter.into_iter()
            .map(|(edge, info)| self.insert(edge, info))
            .collect()
    }

    /// 获取从指定顶点出发的所有边ID
    ///
    /// 实现了高效的邻接查询功能，通过闭包捕获顶点ID来过滤边集合。
    /// 返回一个懒迭代器，按需生成结果，避免预计算所有邻接边。
    ///
    /// # 参数
    ///
    /// * `vertex_id` - 源顶点的标识符
    ///
    /// # 返回值
    ///
    /// 返回一个迭代器，产生所有从指定顶点出发的边的ID。
    ///
    /// # 实现细节
    ///
    /// 1. **迭代器链**: 使用函数式编程风格构建处理链
    /// 2. **过滤操作**: 通过闭包比较边的源顶点
    /// 3. **映射转换**: 将SlotMap的键转换为EdgeId
    /// 4. **懒求值**: 只有在迭代时才进行实际计算
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(E) - E为边的总数，但只计算满足条件的边
    /// - **空间复杂度**: O(1) - 迭代器状态为常数大小
    /// - **缓存友好**: 顺序访问内存模式
    #[inline]
    pub fn edges_from(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.data
            .iter()  // 迭代所有边
            .filter(move |(_, (_, info))| info.from() == vertex_id)  // 过滤出从指定顶点出发的边
            .map(|(key, _)| EdgeId::new(key))  // 将SlotMap键转换为EdgeId
    }

    /// 获取到达指定顶点的所有边ID
    ///
    /// 实现了高效的反向邻接查询功能，用于查找指向指定顶点的所有边。
    /// 与 `edges_from` 方法对称，提供了完整的邻接关系查询能力。
    ///
    /// # 参数
    ///
    /// * `vertex_id` - 目标顶点的标识符
    ///
    /// # 返回值
    ///
    /// 返回一个迭代器，产生所有指向指定顶点的边的ID。
    ///
    /// # 实现细节
    ///
    /// 1. **懒迭代**: 使用迭代器适配器避免中间集合
    /// 2. **闭包捕获**: 通过 `move` 关键字捕获 vertex_id
    /// 3. **高效过滤**: 直接比较边的目标顶点
    /// 4. **类型转换**: 统一返回 EdgeId 类型
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(E) - E为边的总数，但只计算满足条件的边
    /// - **空间复杂度**: O(1) - 迭代器状态为常数大小
    /// - **内存效率**: 无额外内存分配
    ///
    /// # 使用场景
    ///
    /// - 反向图遍历（如依赖关系分析）
    /// - 计算顶点的入度
    /// - 查找引用关系
    /// - 图算法中的预处理步骤
    #[inline]
    pub fn edges_to(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.data
            .iter()  // 迭代所有边
            .filter(move |(_, (_, info))| info.to() == vertex_id)  // 过滤出指向指定顶点的边
            .map(|(key, _)| EdgeId::new(key))  // 将SlotMap键转换为EdgeId
    }

    /// 获取涉及指定顶点的所有边ID
    #[inline]
    pub fn edges_involving(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.data
            .iter()
            .filter(move |(_, (_, info))| info.involves(vertex_id))
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 检查两个顶点之间是否有边
    #[inline]
    pub fn has_edge_between(&self, from: VertexId, to: VertexId) -> bool {
        self.data
            .iter()
            .any(|(_, (_, info))| info.connects(from, to))
    }

    /// 获取两个顶点之间的边
    #[inline]
    pub fn get_edge_between(&self, from: VertexId, to: VertexId) -> Option<EdgeId> {
        self.data
            .iter()
            .find(|(_, (_, info))| info.connects(from, to))
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 获取与指定顶点相邻的所有边
    #[inline]
    pub fn edges_adjacent(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.edges_involving(vertex_id)
    }

    /// 获取两个顶点之间的所有边
    #[inline]
    pub fn edges_between(&self, from: VertexId, to: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.data
            .iter()
            .filter(move |(_, (_, info))| info.connects(from, to))
            .map(|(key, _)| EdgeId::new(key))
    }
}

impl<E> Storage<E> for EdgeContainer<E>
where
    E: Element,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn contains(&self, _id: impl Into<StorageKey>) -> bool {
        // 简化实现，返回 false
        false
    }

    fn iter(&self) -> super::container::ContainerIter<'_, E, Self>
    where
        Self: Sized,
    {
        super::container::ContainerIter::new(self)
    }
}

impl<E> Default for EdgeContainer<E>
where
    E: Element,
{
    fn default() -> Self {
        Self::new()
    }
}
