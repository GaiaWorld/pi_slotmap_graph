/**
 * # SlotMapGraph - 基于 pi_slotmap 的高性能图库
 *
 * 本模块提供了一个基于 `pi_slotmap::SlotMap` 实现的高性能图数据结构。
 * 严格参照 `graph-api-simplegraph` 的模块结构，提供完整的图操作接口。
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