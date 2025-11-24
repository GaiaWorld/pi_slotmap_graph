/**
 * # 引用层 (Reference Layer)
 *
 * 本层重新导出图元素的引用系统，提供对图元素的安全访问和操作。
 * 基于现有的 graph.rs 中的引用实现，确保类型安全和性能。
 *
 * ## 设计原则
 *
 * - **安全性**: 通过引用系统防止悬垂指针和无效访问
 * - **一致性**: 与 graph-api-lib 标准保持一致
 * - **类型安全**: 强类型系统防止类型错误
 * - **性能**: 零成本抽象，运行时无额外开销
 *
 * ## 核心组件
 *
 * - [`VertexReference<'graph, Graph>`]: 顶点引用，提供对顶点的安全访问
 * - [`VertexReferenceMut<'graph, Graph>`]: 可变顶点引用
 * - [`EdgeReference<'graph, Graph>`]: 边引用，提供对边的安全访问
 * - [`EdgeReferenceMut<'graph, Graph>`]: 可变边引用
 *
 * ## 特性
 *
 * ### 引用类型
 * - **不可变引用**: 只读访问图元素，保证数据一致性
 * - **可变引用**: 允许修改图元素，提供更新操作
 * - **引用转换**: 支持与 ElementId 的转换
 *
 * ### 安全保证
 * - **生命周期绑定**: 引用与图的生命周期绑定，防止悬垂指针
 * - **借用检查**: 利用Rust的借用检查器防止数据竞争
 * - **类型安全**: 强类型系统防止类型错误
 *
 * ## 使用示例
 *
 * ```rust
 * use slotmap_graph::reference::{VertexReference, EdgeReference};
 * use slotmap_graph::SlotMapGraph;
 *
 * let mut graph = SlotMapGraph::new();
 * let alice = graph.add_vertex("Alice");
 * let bob = graph.add_vertex("Bob");
 * let friendship = graph.add_edge("friends", alice, bob);
 *
 * // 引用类型通过图的方法获取和使用
 * let vertex_count = graph.vertex_count();
 * let edge_count = graph.edge_count();
 * ```
 */

// 重新导出主要的引用类型
pub use crate::graph::{
    VertexReference, VertexReferenceMut, EdgeReference, EdgeReferenceMut
};