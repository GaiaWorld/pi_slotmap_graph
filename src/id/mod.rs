/**
 * # 标识符系统 (ID System)
 *
 * 本模块提供了类型安全的标识符系统，基于 `pi_slotmap::DefaultKey` 实现。
 *
 * ## 组件
 *
 * - [`VertexId`](vertex_id::VertexId): 顶点标识符
 * - [`EdgeId`](edge_id::EdgeId): 边标识符
 * - [`EdgeInfo`](edge_info::EdgeInfo): 边连接信息
 *
 * ## 设计原则
 *
 * 1. **类型安全**: 每个ID类型都是强类型的，防止混用
 * 2. **高性能**: 基于 `pi_slotmap` 的 O(1) 操作
 * 3. **内存效率**: 紧凑的存储和自动重用
 * 4. **兼容性**: 与 `graph-api-lib` 的 `ElementId` 自动转换
 *
 * ## 使用示例
 *
 * ```rust
 * use slotmap_graph::id::{VertexId, EdgeId};
 * use pi_slotmap::DefaultKey;
 *
 * // 创建ID
 * let vertex_id = VertexId::new(DefaultKey::default());
 * let edge_id = EdgeId::new(DefaultKey::default());
 *
 * // 获取底层键
 * let key = vertex_id.key();
 *
 * // 与ElementId转换
 * use graph_api_lib::ElementId;
 * let element_id: ElementId<_> = vertex_id.into();
 * ```
 */

pub mod vertex_id;
pub mod edge_id;
pub mod edge_info;

// 重新导出主要类型
pub use vertex_id::VertexId;
pub use edge_id::EdgeId;
pub use edge_info::EdgeInfo;

use graph_api_lib::ElementId;

/// 为VertexId实现到ElementId的转换
impl<Graph> From<VertexId> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph<VertexId = VertexId>,
{
    fn from(id: VertexId) -> Self {
        ElementId::Vertex(id)
    }
}

/// 为EdgeId实现到ElementId的转换
impl<Graph> From<EdgeId> for ElementId<Graph>
where
    Graph: graph_api_lib::Graph<EdgeId = EdgeId>,
{
    fn from(id: EdgeId) -> Self {
        ElementId::Edge(id)
    }
}

/// ID验证和转换的辅助trait
pub trait IdExt {
    /// 验证ID是否有效（对应元素是否存在）
    fn is_valid(&self) -> bool;

    /// 转换为usize（用于调试和哈希）
    fn as_usize(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pi_slotmap::DefaultKey;
    use graph_api_lib::ElementId;

    #[test]
    fn test_id_creation() {
        let key = DefaultKey::default();
        let vertex_id = VertexId::new(key);
        let edge_id = EdgeId::new(key);

        assert_eq!(vertex_id.key(), key);
        assert_eq!(edge_id.key(), key);
    }

    #[test]
    fn test_element_id_conversion() {
        type TestGraph = super::super::graph::SlotMapGraph<(), ()>;

        let vertex_id = VertexId::new(DefaultKey::default());
        let edge_id = EdgeId::new(DefaultKey::default());

        let element_vertex_id: ElementId<TestGraph> = vertex_id.into();
        let element_edge_id: ElementId<TestGraph> = edge_id.into();

        match element_vertex_id {
            ElementId::Vertex(id) => assert_eq!(id, vertex_id),
            _ => panic!("Expected Vertex ID"),
        }

        match element_edge_id {
            ElementId::Edge(id) => assert_eq!(id, edge_id),
            _ => panic!("Expected Edge ID"),
        }
    }
}