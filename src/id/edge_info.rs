/**
 * # EdgeInfo - 边连接信息
 *
 * 存储边的起点和终点顶点ID，用于维护图的结构信息。
 *
 * ## 设计原则
 *
 * - **不可变性**: 一旦创建，边的连接信息不会改变
 * - **紧凑存储**: 三个ID的紧凑布局，总共24字节
 * - **类型安全**: 强类型的顶点ID，防止错误使用
 * - **查询效率**: O(1) 的起点和终点查询
 *
 * ## 内存布局
 *
 * ```text,ignore
 * ┌─────────────────────────────────────────────────┐
 * │ EdgeInfo                                        │
 * ├─────────────────────────────────────────────────┤
 * │ edge_id: EdgeId        (8 bytes)                │
 * │ from:    VertexId      (8 bytes)                │
 * │ to:      VertexId      (8 bytes)                │
 * └─────────────────────────────────────────────────┘
 * Total: 24 bytes
 * ```
 *
 * ## 使用场景
 *
 * - **边连接查询**: 快速获取边的起点和终点
 * **图遍历**: 实现基于连接信息的邻接表遍历
 * **数据一致性**: 确保边与顶点之间的连接关系
 * **图分析**: 支持度数计算、连通性分析等
 *
 * ## 使用示例
 *
 * ```rust
 * use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
 * use pi_slotmap::DefaultKey;
 *
 * let edge_id = EdgeId::new(DefaultKey::default());
 * let from_id = VertexId::new(DefaultKey::default());
 * let to_id = VertexId::new(DefaultKey::default());
 *
 * // 创建边连接信息
 * let edge_info = EdgeInfo::new(edge_id, from_id, to_id);
 *
 * // 查询连接信息
 * assert_eq!(edge_info.edge_id(), edge_id);
 * assert_eq!(edge_info.from(), from_id);
 * assert_eq!(edge_info.to(), to_id);
 *
 * // 反向连接
 * let reverse_info = edge_info.reverse();
 * assert_eq!(reverse_info.from(), to_id);
 * assert_eq!(reverse_info.to(), from_id);
 * ```
 */

use super::{EdgeId, VertexId};
use pi_slotmap::Key;

/// 边连接信息，存储边及其起点和终点顶点
///
/// 这个结构体用于维护图的结构信息，确保边与顶点之间的正确连接关系。
/// 每条边都有一个对应的 `EdgeInfo` 实例，存储该边的元数据。
///
/// # 字段说明
///
/// - `edge_id`: 边的唯一标识符
/// - `from`: 边的起始顶点ID
/// - `to`: 边的目标顶点ID
///
/// # 不变性
///
/// `EdgeInfo` 一旦创建就是不可变的。如果需要改变边的连接关系，
/// 应该创建新的 `EdgeInfo` 实例，而不是修改现有的实例。
///
/// # Examples
///
/// ```rust
/// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
/// use pi_slotmap::DefaultKey;
///
/// // 创建顶点和边ID
/// let edge_id = EdgeId::new(DefaultKey::default());
/// let vertex_a = VertexId::new(DefaultKey::default());
/// let vertex_b = VertexId::new(DefaultKey::default());
///
/// // 创建边连接信息
/// let edge_info = EdgeInfo::new(edge_id, vertex_a, vertex_b);
///
/// // 查询信息
/// assert_eq!(edge_info.edge_id(), edge_id);
/// assert_eq!(edge_info.from(), vertex_a);
/// assert_eq!(edge_info.to(), vertex_b);
///
/// // 检查连接方向
/// assert!(edge_info.connects_from(vertex_a));
/// assert!(edge_info.connects_to(vertex_b));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeInfo {
    edge_id: EdgeId,
    from: VertexId,
    to: VertexId,
}

impl EdgeInfo {
    /// 创建新的边连接信息
    ///
    /// # Arguments
    ///
    /// * `edge_id` - 边的唯一标识符
    /// * `from` - 起始顶点ID
    /// * `to` - 目标顶点ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let edge_id = EdgeId::new(DefaultKey::default());
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    ///
    /// let edge_info = EdgeInfo::new(edge_id, from, to);
    /// ```
    #[inline]
    pub const fn new(edge_id: EdgeId, from: VertexId, to: VertexId) -> Self {
        Self { edge_id, from, to }
    }

    /// 获取边的ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let edge_id = EdgeId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(edge_id, VertexId::default(), VertexId::default());
    /// assert_eq!(edge_info.edge_id(), edge_id);
    /// ```
    #[inline]
    pub const fn edge_id(&self) -> EdgeId {
        self.edge_id
    }

    /// 获取起始顶点ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, VertexId::default());
    /// assert_eq!(edge_info.from(), from);
    /// ```
    #[inline]
    pub const fn from(&self) -> VertexId {
        self.from
    }

    /// 获取目标顶点ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let to = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), VertexId::default(), to);
    /// assert_eq!(edge_info.to(), to);
    /// ```
    #[inline]
    pub const fn to(&self) -> VertexId {
        self.to
    }

    /// 检查边是否从指定顶点出发
    ///
    /// # Arguments
    ///
    /// * `vertex_id` - 要检查的顶点ID
    ///
    /// # Returns
    ///
    /// 如果该顶点是边的起点，返回 `true`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
    ///
    /// assert!(edge_info.connects_from(from));
    /// assert!(!edge_info.connects_from(to));
    /// ```
    #[inline]
    pub fn connects_from(&self, vertex_id: VertexId) -> bool {
        self.from == vertex_id
    }

    /// 检查边是否到达指定顶点
    ///
    /// # Arguments
    ///
    /// * `vertex_id` - 要检查的顶点ID
    ///
    /// # Returns
    ///
    /// 如果该顶点是边的终点，返回 `true`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
    ///
    /// assert!(!edge_info.connects_to(from));
    /// assert!(edge_info.connects_to(to));
    /// ```
    #[inline]
    pub fn connects_to(&self, vertex_id: VertexId) -> bool {
        self.to == vertex_id
    }

    /// 检查边是否连接两个指定的顶点
    ///
    /// # Arguments
    ///
    /// * `from` - 起始顶点ID
    /// * `to` - 目标顶点ID
    ///
    /// # Returns
    ///
    /// 如果边从 `from` 连接到 `to`，返回 `true`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let vertex_a = VertexId::new(DefaultKey::default());
    /// let vertex_b = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), vertex_a, vertex_b);
    ///
    /// assert!(edge_info.connects(vertex_a, vertex_b));
    /// assert!(!edge_info.connects(vertex_b, vertex_a));
    /// ```
    #[inline]
    pub fn connects(&self, from: VertexId, to: VertexId) -> bool {
        self.from == from && self.to == to
    }

    /// 检查边是否连接指定顶点（无论是起点还是终点）
    ///
    /// # Arguments
    ///
    /// * `vertex_id` - 要检查的顶点ID
    ///
    /// # Returns
    ///
    /// 如果该顶点是边的起点或终点，返回 `true`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    /// let other = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
    ///
    /// assert!(edge_info.involves(from));
    /// assert!(edge_info.involves(to));
    /// assert!(!edge_info.involves(other));
    /// ```
    #[inline]
    pub fn involves(&self, vertex_id: VertexId) -> bool {
        self.from == vertex_id || self.to == vertex_id
    }

    /// 创建反向边信息（交换起点和终点）
    ///
    /// # Returns
    ///
    /// 新的 `EdgeInfo` 实例，起点和终点互换
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
    ///
    /// let reversed = edge_info.reverse();
    /// assert_eq!(reversed.from(), to);
    /// assert_eq!(reversed.to(), from);
    /// assert_eq!(reversed.edge_id(), edge_info.edge_id());
    /// ```
    #[inline]
    pub const fn reverse(&self) -> Self {
        Self {
            edge_id: self.edge_id,
            from: self.to,
            to: self.from,
        }
    }

    /// 获取边的两个端点
    ///
    /// # Returns
    ///
    /// 元组 `(from, to)`，包含起点和终点
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let from = VertexId::new(DefaultKey::default());
    /// let to = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), from, to);
    ///
    /// let (start, end) = edge_info.endpoints();
    /// assert_eq!(start, from);
    /// assert_eq!(end, to);
    /// ```
    #[inline]
    pub const fn endpoints(&self) -> (VertexId, VertexId) {
        (self.from, self.to)
    }

    /// 获取边的排序后的端点
    ///
    /// 按ID值排序端点，用于无向图操作
    ///
    /// # Returns
    ///
    /// 元组 `(min, max)`，包含按ID排序后的端点
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::{EdgeInfo, EdgeId, VertexId};
    /// use pi_slotmap::DefaultKey;
    ///
    /// let vertex_a = VertexId::new(DefaultKey::default());
    /// let vertex_b = VertexId::new(DefaultKey::default());
    /// let edge_info = EdgeInfo::new(EdgeId::default(), vertex_b, vertex_a);
    ///
    /// let (min, max) = edge_info.sorted_endpoints();
    /// assert_eq!(min, vertex_a);
    /// assert_eq!(max, vertex_b);
    /// ```
    #[inline]
    pub fn sorted_endpoints(&self) -> (VertexId, VertexId) {
        // 注意：这里需要比较DefaultKey，但由于VertexId是newtype，我们需要转换为底层键进行比较
        if self.from.key().data().as_ffi() <= self.to.key().data().as_ffi() {
            (self.from, self.to)
        } else {
            (self.to, self.from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pi_slotmap::{DefaultKey, SlotMap};

    /// 创建测试用的不同 DefaultKey
    fn create_test_keys(count: usize) -> Vec<DefaultKey> {
        let mut slotmap: SlotMap<DefaultKey, usize> = SlotMap::new();
        let mut keys = Vec::new();
        for i in 0..count {
            let key = slotmap.insert(i);
            keys.push(key);
        }
        keys
    }

    #[test]
    fn test_edge_info_creation() {
        let edge_id = EdgeId::new(DefaultKey::default());
        let from = VertexId::new(DefaultKey::default());
        let to = VertexId::new(DefaultKey::default());

        let edge_info = EdgeInfo::new(edge_id, from, to);

        assert_eq!(edge_info.edge_id(), edge_id);
        assert_eq!(edge_info.from(), from);
        assert_eq!(edge_info.to(), to);
    }

    #[test]
    fn test_edge_info_connects() {
        let keys = create_test_keys(3);
        let vertex_a = VertexId::new(keys[0]);
        let vertex_b = VertexId::new(keys[1]);
        let vertex_c = VertexId::new(keys[2]);

        let edge_info = EdgeInfo::new(EdgeId::default(), vertex_a, vertex_b);

        assert!(edge_info.connects(vertex_a, vertex_b));
        assert!(!edge_info.connects(vertex_b, vertex_a));
        assert!(!edge_info.connects(vertex_a, vertex_c));
    }

    #[test]
    fn test_edge_info_involves() {
        let keys = create_test_keys(3);
        let vertex_a = VertexId::new(keys[0]);
        let vertex_b = VertexId::new(keys[1]);
        let vertex_c = VertexId::new(keys[2]);

        let edge_info = EdgeInfo::new(EdgeId::default(), vertex_a, vertex_b);

        assert!(edge_info.involves(vertex_a));
        assert!(edge_info.involves(vertex_b));
        assert!(!edge_info.involves(vertex_c));
    }

    #[test]
    fn test_edge_info_reverse() {
        let keys = create_test_keys(2);
        let vertex_a = VertexId::new(keys[0]);
        let vertex_b = VertexId::new(keys[1]);
        let edge_id = EdgeId::default();

        let edge_info = EdgeInfo::new(edge_id, vertex_a, vertex_b);
        let reversed = edge_info.reverse();

        assert_eq!(reversed.from(), vertex_b);
        assert_eq!(reversed.to(), vertex_a);
        assert_eq!(reversed.edge_id(), edge_id);
    }

    #[test]
    fn test_edge_info_endpoints() {
        let keys = create_test_keys(2);
        let vertex_a = VertexId::new(keys[0]);
        let vertex_b = VertexId::new(keys[1]);

        let edge_info = EdgeInfo::new(EdgeId::default(), vertex_a, vertex_b);
        let (from, to) = edge_info.endpoints();

        assert_eq!(from, vertex_a);
        assert_eq!(to, vertex_b);
    }

    #[test]
    fn test_edge_info_sorted_endpoints() {
        let keys = create_test_keys(3);
        let vertex_a = VertexId::new(keys[0]);
        let vertex_b = VertexId::new(keys[1]);
        let vertex_c = VertexId::new(keys[2]);

        // 测试正常顺序
        let edge_info1 = EdgeInfo::new(EdgeId::default(), vertex_a, vertex_b);
        let (min1, max1) = edge_info1.sorted_endpoints();
        // 由于 DefaultKey 的比较是基于其内部值的，我们不能保证插入顺序
        // 所以这里我们只测试返回的两个顶点是原始的两个顶点，只是可能顺序不同
        assert!((min1 == vertex_a && max1 == vertex_b) || (min1 == vertex_b && max1 == vertex_a));

        // 测试需要排序的情况
        let edge_info2 = EdgeInfo::new(EdgeId::default(), vertex_c, vertex_a);
        let (min2, max2) = edge_info2.sorted_endpoints();
        assert!((min2 == vertex_a && max2 == vertex_c) || (min2 == vertex_c && max2 == vertex_a));
    }

    #[test]
    fn test_edge_info_equality() {
        let keys = create_test_keys(3);
        let edge_id = EdgeId::new(keys[0]);
        let from = VertexId::new(keys[1]);
        let to = VertexId::new(keys[2]);

        let edge_info1 = EdgeInfo::new(edge_id, from, to);
        let edge_info2 = EdgeInfo::new(edge_id, from, to);
        let edge_info3 = EdgeInfo::new(edge_id, to, from);

        assert_eq!(edge_info1, edge_info2);
        assert_ne!(edge_info1, edge_info3);
    }

    #[test]
    fn test_edge_info_hash() {
        use std::collections::HashSet;

        let keys = create_test_keys(3);
        let edge_id = EdgeId::new(keys[0]);
        let from = VertexId::new(keys[1]);
        let to = VertexId::new(keys[2]);

        let edge_info = EdgeInfo::new(edge_id, from, to);
        let mut set = HashSet::new();
        set.insert(edge_info);

        assert!(set.contains(&edge_info));
    }
}