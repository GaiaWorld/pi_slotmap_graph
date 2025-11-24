/**
 * # SlotMapGraph - 基于 pi_slotmap 的高性能图库
 *
 * 本模块提供了一个基于 `pi_slotmap::SlotMap` 实现的高性能图数据结构。
 * 严格参照 `graph-api-simplegraph` 的模块结构，提供完整的图操作接口。
 *
 * ## 核心设计理念
 *
 * ### 为什么选择 SlotMap？
 *
 * **传统HashMap的问题**：
 * - 指针重用导致的安全隐患（A删除后，B可能获得相同的地址）
 * - 迭代器失效问题（修改HashMap时所有迭代器都可能失效）
 * - 缓存不友好的内存布局
 *
 * **SlotMap的优势**：
 * - **稳定的索引**：删除元素后不会重用索引，避免了悬垂指针
 * - **迭代器安全**：删除操作不会影响其他正在使用的迭代器
 * - **缓存友好**：连续内存布局，提高CPU缓存命中率
 * - **O(1)保证**：所有操作都有严格的时间复杂度保证
 *
 * ## 模块架构
 *
 * 本模块采用分层架构设计，从底层到顶层依次为：
 *
 * ```text,ignore
 * ┌─────────────────────────────────────────────────┐
 * │                Graph Layer                      │
 * │         核心图实现和公共接口                     │
 * └─────────────────────────────────────────────────┘
 *                       │
 * ┌─────────────────────────────────────────────────┐
 * │              Reference Layer                     │
 * │         顶点引用和边引用的实现                    │
 * └─────────────────────────────────────────────────┘
 *                       │
 * ┌─────────────────────────────────────────────────┐
 * │              Iteration Layer                     │
 * │         高效的迭代器实现                         │
 * └─────────────────────────────────────────────────┘
 *                       │
 * ┌─────────────────────────────────────────────────┐
 * │               Storage Layer                      │
 * │        基于 SlotMap 的存储容器                   │
 * └─────────────────────────────────────────────────┘
 *                       │
 * ┌─────────────────────────────────────────────────┐
 │                 ID Layer                         │
 * │        类型安全的 ID 系统                        │
 * └─────────────────────────────────────────────────┘
 * ```
 *
 * ## 核心特性
 *
 * ### 高性能存储
 * - **O(1) 插入**: 基于 SlotMap 的常数时间插入操作
 * - **O(1) 删除**: 基于 SlotMap 的常数时间删除操作
 * - **O(1) 查找**: 基于 SlotMap 的常数时间查找操作
 * - **内存效率**: 紧凑存储，自动重用删除的空间
 *
 * ### 类型安全
 * - **强类型ID**: 独立的 VertexId 和 EdgeId 类型，防止混淆
 * - **泛型支持**: 支持任意类型的顶点权重和边标签
 * - **借用检查**: 利用 Rust 的借用检查器确保内存安全
 *
 * ### 迭代器优化
 * - **零成本抽象**: 迭代器无运行时开销
 * **懒加载**: 按需生成元素，不预计算
 * **链式操作**: 支持迭代器的链式组合
 *
 * ## 使用示例
 *
 * ### 基本操作
 * ```rust
 * use slotmap_graph::SlotMapGraph;
 *
 * // 创建图
 * let mut graph = SlotMapGraph::new();
 *
 * // 添加顶点
 * let alice = graph.add_vertex("Alice");
 * let bob = graph.add_vertex("Bob");
 * let charlie = graph.add_vertex("Charlie");
 *
 * // 添加边
 * let friendship = graph.add_edge("friends", alice, bob);
 * let acquaintance = graph.add_edge("acquaintances", bob, charlie);
 *
 * // 查询顶点
 * if let Some(alice_name) = graph.vertex_weight(alice) {
 *     println!("Alice: {}", alice_name);
 * }
 *
 * // 查询边
 * if let Some(edge_label) = graph.edge_weight(friendship) {
 *     println!("Relationship: {}", edge_label);
 * }
 * ```
 *
 * ### 遍历操作
 * ```rust
 * use graph_api_lib::Direction;
 *
 * // 遍历所有顶点
 * for vertex_ref in graph.vertex_iter() {
 *     println!("Vertex: {}", vertex_ref.value());
 * }
 *
 * // 遍历所有边
 * for edge_ref in graph.edge_iter() {
 *     println!("Edge: {} -> {}", edge_ref.from(), edge_ref.to());
 * }
 *
 * // 查询顶点的邻接边
 * for edge_ref in graph.vertex_edges(alice) {
 *     match edge_ref.direction() {
 *         Direction::Outgoing => {
 *             println!("{} -> {}", alice, edge_ref.to());
 *         },
 *         Direction::Incoming => {
 *             println!("{} <- {}", alice, edge_ref.from());
 *         },
 *     }
 * }
 * ```
 *
 * ### 可变操作
 * ```rust
 * // 修改顶点数据
 * if let Some(mut alice_ref) = graph.vertex_reference_mut(alice) {
 *     *alice_ref.value_mut = "Alice Smith".to_string();
 * }
 *
 * // 修改边数据
 * if let Some(mut friend_ref) = graph.edge_reference_mut(friendship) {
 *     *friend_ref.value_mut = "best friends".to_string();
 * }
 *
 * // 批量修改
 * for mut vertex_ref in graph.vertex_iter_mut() {
 *     vertex_ref.value_mut().push_str(" (modified)");
 * }
 * ```
 *
 * ## 模块导出
 *
 * ### 公共接口
 * - [`SlotMapGraph`]: 核心图实现
 * - [`VertexId`]: 顶点唯一标识符
 * - [`EdgeId`]: 边唯一标识符
 * - [`VertexContainer`]: 顶点存储容器
 * - [`EdgeContainer`]: 边存储容器
 *
 * ### 引用系统
 * - [`VertexReference`]: 顶点引用
 * - [`EdgeReference`]: 边引用
 *
 * ### 迭代器系统
 * - [`VertexIter`]: 顶点迭代器
 * - [`EdgeIter`]: 边迭代器
 * - [`VertexIterMut`]: 可变顶点迭代器
 * - [`EdgeIterMut`]: 可变边迭代器
 *
 * ## 性能特征
 *
 * ### 时间复杂度
 * - **插入操作**: O(1) - 顶点和边的插入都是常数时间
 * - **删除操作**: O(1) - 删除顶点或边及其相关连接
 * - **查询操作**: O(1) - 通过ID直接访问顶点或边
 * - **邻接查询**: O(k) - k为顶点的度数（邻居数量）
 * - **遍历操作**: O(n) - n为顶点或边的总数
 *
 * ### 空间复杂度
 * - **顶点存储**: O(V) - V为顶点数量，每个顶点8字节开销
 * - **边存储**: O(E) - E为边数量，每条边24字节开销（含连接信息）
 * - **内存局部性**: 连续内存布局，缓存友好
 * - **内存重用**: 删除后的空间自动重用，无内存泄漏
 *
 * ## 内存安全保证
 *
 * ### 借用检查
 * - **不可变引用**: 允许同时存在多个不可变引用
 * - **可变引用**: 严格的独占可变引用，防止数据竞争
 * - **生命周期**: 编译时验证引用有效性
 * - **ID稳定性**: 删除操作不会使现有ID失效
 *
 * ### 错误处理
 * - **ID验证**: 所有ID操作都会验证有效性
 * - **边界检查**: 自动防止越界访问
 * - **类型安全**: 编译时防止类型混淆
 * - **异常安全**: 操作失败时保持数据一致性
 *
 * ## 高级用法示例
 *
 * ### 构建复杂网络
 * ```rust
 * use slotmap_graph::SlotMapGraph;
 *
 * // 社交网络示例
 * let mut social_graph = SlotMapGraph::new();
 *
 * // 添加用户节点
 * let alice = social_graph.add_vertex(("Alice", 25));
 * let bob = social_graph.add_vertex(("Bob", 30));
 * let charlie = social_graph.add_vertex(("Charlie", 28));
 *
 * // 添加关系边
 * let friendship_ab = social_graph.add_edge("friend", alice, bob);
 * let friendship_bc = social_graph.add_edge("friend", bob, charlie);
 * let follows_ac = social_graph.add_edge("follows", alice, charlie);
 *
 * // 查找Alice的朋友
 * for edge_ref in social_graph.outgoing_edges(alice) {
 *     let friend_id = edge_ref.head();
 *     if let Some(friend_ref) = social_graph.vertex(friend_id) {
 *         let (name, age) = friend_ref.weight();
 *         println!("Alice的朋友: {} ({}岁)", name, age);
 *     }
 * }
 * ```
 *
 * ### 图算法示例
 * ```rust
 * // 简单的广度优先搜索
 * fn bfs_shortest_path<V, E>(
 *     graph: &SlotMapGraph<V, E>,
 *     start: VertexId,
 *     goal: VertexId,
 * ) -> Option<Vec<VertexId>> {
 *     let mut visited = std::collections::HashSet::new();
 *     let mut queue = std::collections::VecDeque::new();
 *     let mut parent = std::collections::HashMap::new();
 *
 *     queue.push_back(start);
 *     visited.insert(start);
 *
 *     while let Some(current) = queue.pop_front() {
 *         if current == goal {
 *             // 重建路径
 *             let mut path = vec![goal];
 *             while let Some(&p) = parent.get(&path.last().unwrap()) {
 *                 path.push(p);
 *             }
 *             path.reverse();
 *             return Some(path);
 *         }
 *
 *         for edge_ref in graph.outgoing_edges(current) {
 *             let neighbor = edge_ref.head();
 *             if !visited.contains(&neighbor) {
 *                 visited.insert(neighbor);
 *                 parent.insert(neighbor, current);
 *                 queue.push_back(neighbor);
 *             }
 *         }
 *     }
 *
 *     None
 * }
 * ```
 */

// 核心图实现
pub mod graph;

// 分层模块结构
pub mod id;
pub mod storage;
pub mod reference;
pub mod iteration;

// 主要类型导出
pub use graph::SlotMapGraph;
pub use id::{EdgeId, EdgeInfo, VertexId};
pub use storage::{EdgeContainer, VertexContainer};
pub use reference::{VertexReference, VertexReferenceMut, EdgeReference, EdgeReferenceMut};

#[cfg(test)]
mod test {
    use crate::SlotMapGraph;
    use graph_api_test::test_suite;

    test_suite!(SlotMapGraph::new());
}