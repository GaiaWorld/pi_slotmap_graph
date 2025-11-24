/**
 * # EdgeId - 边标识符
 *
 * 基于 `pi_slotmap::DefaultKey` 的类型安全的边标识符实现。
 *
 * ## 特性
 *
 * - **类型安全**: 与 `VertexId` 类型隔离，防止混用
 * - **高性能**: 基于 `pi_slotmap` 的底层实现
 * - **内存效率**: 8字节存储，自动重用
 * - **调试友好**: 实现了 `Debug`, `Display`, `Hash` 等常用trait
 *
 * ## 内部表示
 *
 * ```rust,ignore
 * pub struct EdgeId(pub(crate) DefaultKey);
 * ```
 *
 * 与 `VertexId` 类似，存储为 `DefaultKey` 的newtype包装器，
 * 但在类型层面与顶点ID区分，提供编译时的类型安全保证。
 *
 * ## 使用示例
 *
 * ```rust
 * use slotmap_graph::id::EdgeId;
 * use pi_slotmap::DefaultKey;
 *
 * // 创建边ID
 * let key = DefaultKey::default();
 * let edge_id = EdgeId::new(key);
 *
 * // 获取底层键
 * let key = edge_id.key();
 *
 * // 比较
 * assert_eq!(edge_id, EdgeId::new(key));
 *
 * // 哈希和比较
 * use std::collections::HashSet;
 * let mut set = HashSet::new();
 * set.insert(edge_id);
 * ```
 */

use pi_slotmap::{DefaultKey, Key};
use std::fmt;

/// 边标识符，基于 `pi_slotmap::DefaultKey` 实现
///
/// 提供类型安全的边标识，防止与顶点ID混淆。
/// 内部使用 `DefaultKey` 作为底层存储，提供高性能的插入、删除和查找操作。
///
/// # Examples
///
/// ```rust
/// use slotmap_graph::id::EdgeId;
/// use pi_slotmap::DefaultKey;
///
/// // 创建ID
/// let key = DefaultKey::default();
/// let edge_id = EdgeId::new(key);
///
/// // 访问底层键
/// assert_eq!(edge_id.key(), key);
///
/// // 克隆和比较
/// let cloned = edge_id;
/// assert_eq!(edge_id, cloned);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EdgeId(pub(crate) DefaultKey);

impl EdgeId {
    /// 创建新的边ID
    ///
    /// # Arguments
    /// * `key` - 底层的 `DefaultKey`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::EdgeId;
    /// use pi_slotmap::DefaultKey;
    ///
    /// let key = DefaultKey::default();
    /// let edge_id = EdgeId::new(key);
    /// ```
    #[inline]
    pub fn new(key: DefaultKey) -> Self {
        Self(key)
    }

    /// 获取底层的 `DefaultKey`
    ///
    /// 用于与底层存储容器交互。
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::EdgeId;
    /// use pi_slotmap::DefaultKey;
    ///
    /// let key = DefaultKey::default();
    /// let edge_id = EdgeId::new(key);
    ///
    /// assert_eq!(edge_id.key(), key);
    /// ```
    #[inline]
    pub fn key(&self) -> DefaultKey {
        self.0
    }

    /// 检查ID是否为默认值
    ///
    /// 默认值表示这是一个未初始化的ID。
    ///
    /// # Examples
    ///
    /// ```rust
    /// use slotmap_graph::id::EdgeId;
    ///
    /// let default_id = EdgeId::default();
    /// assert!(default_id.is_default());
    /// ```
    #[inline]
    pub fn is_default(&self) -> bool {
        self.0 == DefaultKey::default()
    }
}

impl fmt::Display for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{}", self.0.data().as_ffi())
    }
}

/// 从 `DefaultKey` 转换为 `EdgeId`
impl From<DefaultKey> for EdgeId {
    #[inline]
    fn from(key: DefaultKey) -> Self {
        Self(key)
    }
}

/// 从 `EdgeId` 转换为 `DefaultKey`
impl From<EdgeId> for DefaultKey {
    #[inline]
    fn from(id: EdgeId) -> Self {
        id.0
    }
}

impl super::IdExt for EdgeId {
    /// 验证EdgeId是否有效
    ///
    /// 对于EdgeId来说，有效性检查需要在具体的图实例中进行，
    /// 这里返回true表示格式上是有效的。
    #[inline]
    fn is_valid(&self) -> bool {
        self.0 != DefaultKey::default()
    }

    /// 转换为usize用于调试和哈希
    #[inline]
    fn as_usize(&self) -> usize {
        // 转换为u64然后转为usize，确保在64位和32位系统上都能工作
        self.0.data().as_ffi() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::IdExt;
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
    fn test_edge_id_creation() {
        let key = DefaultKey::default();
        let edge_id = EdgeId::new(key);

        assert_eq!(edge_id.key(), key);
    }

    #[test]
    fn test_edge_id_equality() {
        let key = DefaultKey::default();
        let id1 = EdgeId::new(key);
        let id2 = EdgeId::new(key);

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_edge_id_clone() {
        let key = DefaultKey::default();
        let edge_id = EdgeId::new(key);
        let cloned = edge_id;

        assert_eq!(edge_id, cloned);
    }

    #[test]
    fn test_edge_id_hash() {
        use std::collections::HashSet;

        let key = DefaultKey::default();
        let edge_id = EdgeId::new(key);

        let mut set = HashSet::new();
        set.insert(edge_id);

        assert!(set.contains(&edge_id));
    }

    #[test]
    fn test_edge_id_display() {
        let key = DefaultKey::default();
        let edge_id = EdgeId::new(key);

        let display_str = format!("{}", edge_id);
        assert!(display_str.starts_with('E'));
    }

    #[test]
    fn test_from_default_key() {
        let key = DefaultKey::default();
        let edge_id: EdgeId = key.into();

        assert_eq!(edge_id.key(), key);
    }

    #[test]
    fn test_into_default_key() {
        let key = DefaultKey::default();
        let edge_id = EdgeId::new(key);
        let converted_key: DefaultKey = edge_id.into();

        assert_eq!(converted_key, key);
    }

    #[test]
    fn test_id_ext() {
        let keys = create_test_keys(1);
        let key = keys[0];
        let edge_id = EdgeId::new(key);

        assert!(edge_id.is_valid());
        assert_eq!(edge_id.as_usize(), key.data().as_ffi() as usize);
    }

    #[test]
    fn test_default_id() {
        let default_id = EdgeId::default();
        assert!(default_id.is_default());
    }

    #[test]
    fn test_distinct_from_vertex_id() {
        use super::super::vertex_id::VertexId;

        let keys = create_test_keys(1);
        let key = keys[0];
        let vertex_id = VertexId::new(key);
        let edge_id = EdgeId::new(key);

        // VertexId 和 EdgeId 是不同的类型，即使包含相同的 DefaultKey
        // 但是它们可以通过 .key() 方法获取相同的底层键
        assert_eq!(vertex_id.key(), edge_id.key());

        // 由于类型不同，VertexId 和 EdgeId 不能直接比较
        // 这里我们测试它们的底层键是相同的
        assert_eq!(vertex_id.key(), edge_id.key());
    }
}