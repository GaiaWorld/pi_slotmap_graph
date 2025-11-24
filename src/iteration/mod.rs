/**
 * # 迭代层 (Iteration Layer)
 *
 * 本层重新导出高效的图元素迭代器，支持各种遍历模式和查询操作。
 * 基于 graph-api-lib 标准迭代器，与现有实现保持一致。
 */

/// 迭代层标记类型，用于模块导出
pub struct IterationLayer;

// 重新导出标准迭代器类型，这些类型由 Graph trait 提供
// 实际的迭代器实例通过图的方法获取，如：
// - graph.vertices() -> 顶点迭代器
// - graph.edges() -> 边迭代器
// - graph.vertex_edges(vertex_id) -> 顶点邻接边迭代器