/**
 * # 存储层 (Storage Layer)
 *
 * 本模块提供高性能的存储容器实现，基于 `pi_slotmap::SlotMap`。
 *
 * ## 设计原则
 *
 * - **高性能**: 利用 SlotMap 的 O(1) 插入、删除和查找
 * - **内存效率**: 紧凑的存储和自动重用机制
 * - **类型安全**: 强类型的泛型容器
 * - **可扩展**: 支持自定义存储策略
 *
 * ## 核心组件
 *
 * - [`VertexContainer<V>`](vertex::VertexContainer): 顶点存储容器
 * - [`EdgeContainer<E>`](edge::EdgeContainer): 边存储容器
 * - [`Container`]: 通用存储接口
 *
 * ## 特性
 *
 * ### 顶点存储
 * - O(1) 插入、删除、查找
 * - 支持迭代器遍历
 * - 自动内存重用
 *
 * ### 边存储
 * - O(1) 插入、删除、查找
 * - 内置连接信息管理
 * - 支持高效的邻接查询
 * - 支持方向过滤（入边/出边/相邻边）
 *
 * ## 使用示例
 *
 * ```rust
 * use slotmap_graph::storage::{VertexContainer, EdgeContainer};
 * use slotmap_graph::id::{VertexId, EdgeId};
 *
 * // 创建存储容器
 * let mut vertices = VertexContainer::new();
 * let mut edges = EdgeContainer::new();
 *
 * // 添加顶点
 * let alice = vertices.insert("Alice");
 * let bob = vertices.insert("Bob");
 *
 * // 添加边
 * let friendship = edges.insert("friends", alice, bob);
 *
 * // 查询顶点
 * if let Some(name) = vertices.get(alice) {
 *     println!("Found: {}", name);
 * }
 *
 * // 查询边连接信息
 * if let Some(edge_info) = edges.get_connection(friendship) {
 *     println!("Edge from {:?} to {:?}", edge_info.from(), edge_info.to());
 * }
 * ```
 */

pub mod vertex;
pub mod edge;
pub mod container;

// 重新导出主要类型
pub use vertex::VertexContainer;
pub use edge::EdgeContainer;
pub use container::{Container, ContainerIter};

/// 存储容器的通用操作接口
///
/// 定义了所有存储容器应该实现的基本操作。
pub trait Storage<T> {
    /// 获取存储的元素数量
    fn len(&self) -> usize;

    /// 检查存储是否为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 清空所有元素
    fn clear(&mut self);

    /// 检查是否包含指定ID
    fn contains(&self, id: impl Into<StorageKey>) -> bool;

    /// 迭代所有元素
    fn iter(&self) -> ContainerIter<'_, T, Self>
    where
        Self: Sized;
}

/// 存储键的通用类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageKey(pub(crate) pi_slotmap::DefaultKey);

impl StorageKey {
    /// 创建新的存储键
    #[inline]
    pub fn new(key: pi_slotmap::DefaultKey) -> Self {
        Self(key)
    }

    /// 获取底层键
    #[inline]
    pub fn key(&self) -> pi_slotmap::DefaultKey {
        self.0
    }
}

/// 存储操作的错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    /// 指定的键不存在
    KeyNotFound(StorageKey),
    /// 存储已满
    StorageFull,
    /// 无效的操作
    InvalidOperation(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::KeyNotFound(key) => write!(f, "Key not found: {:?}", key),
            StorageError::StorageFull => write!(f, "Storage is full"),
            StorageError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

/// 存储容器的统计信息
#[derive(Debug, Clone, PartialEq)]
pub struct StorageStats {
    /// 元素总数
    pub elements: usize,
    /// 已删除的空槽数量
    pub tombstones: usize,
    /// 内存使用量（字节）
    pub memory_bytes: usize,
    /// 碎片率（0.0 - 1.0）
    pub fragmentation: f32,
}

impl StorageStats {
    /// 创建新的统计信息
    pub fn new() -> Self {
        Self {
            elements: 0,
            tombstones: 0,
            memory_bytes: 0,
            fragmentation: 0.0,
        }
    }

    /// 获取填充率
    pub fn fill_rate(&self) -> f32 {
        if self.elements + self.tombstones == 0 {
            1.0
        } else {
            self.elements as f32 / (self.elements + self.tombstones) as f32
        }
    }
}

impl Default for StorageStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pi_slotmap::DefaultKey;

    #[test]
    fn test_storage_key() {
        let key = DefaultKey::default();
        let storage_key = StorageKey::new(key);

        assert_eq!(storage_key.key(), key);
    }

    #[test]
    fn test_storage_stats() {
        let mut stats = StorageStats::new();

        assert_eq!(stats.elements, 0);
        assert_eq!(stats.fill_rate(), 1.0);

        stats.elements = 10;
        stats.tombstones = 5;
        assert_eq!(stats.fill_rate(), 10.0 / 15.0);
    }

    #[test]
    fn test_storage_error() {
        let key = StorageKey::new(DefaultKey::default());
        let error = StorageError::KeyNotFound(key);

        let error_str = format!("{}", error);
        // 检查错误消息包含预期的关键字，而不是精确匹配具体的键值
        assert!(error_str.contains("Key not found: StorageKey(DefaultKey("));
        assert!(error_str.ends_with("v0))"));
    }

    #[test]
    fn test_default_storage_stats() {
        let stats: StorageStats = Default::default();

        assert_eq!(stats.elements, 0);
        assert_eq!(stats.tombstones, 0);
        assert_eq!(stats.memory_bytes, 0);
        assert_eq!(stats.fragmentation, 0.0);
    }
}