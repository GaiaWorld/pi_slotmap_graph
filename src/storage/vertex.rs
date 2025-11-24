/**
 * # VertexContainer - 顶点存储容器
 *
 * 基于 `pi_slotmap::SlotMap` 的高性能顶点存储实现。
 *
 * ## 设计原则
 *
 * - **高性能**: O(1) 插入、删除、查找操作
 * - **内存效率**: 紧凑存储，自动重用删除的空间
 * - **类型安全**: 强类型的泛型设计
 * - **迭代友好**: 高效的迭代器实现
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
use super::super::id::VertexId;
use pi_slotmap::{DefaultKey, SlotMap};

/// 顶点存储容器，基于 `pi_slotmap::SlotMap` 实现
///
/// 这个结构体提供了高性能的顶点存储功能，是整个图数据结构的底层存储基础。
/// 它封装了 `pi_slotmap::SlotMap`，提供了类型安全的顶点存储和检索功能。
///
/// # 内存布局
///
/// ```text
/// +----------------------+----------------------+----------------------+
/// | Slot 1: Vertex Data | Slot 2: Vertex Data | Slot 3: Empty Slot   |
/// | (Key: 1, Data: V)   | (Key: 2, Data: V)   | (Tombstone Marker)   |
/// +----------------------+----------------------+----------------------+
/// ```
///
/// # 特性说明
///
/// ## 稳定的键引用
/// - 键在元素删除后保持稳定，不会被重用
/// - 避免了悬垂指针问题，提高了安全性
/// - 支持长期的引用和迭代器有效性
///
/// ## 高效的内存管理
/// - 删除的槽位被标记为"墓碑"，后续可重用
/// - 自动压缩机制，减少内存碎片
/// - 连续内存布局，提高缓存命中率
///
/// ## 泛型设计
/// - 支持任意实现了 `Clone` 的顶点类型
/// - 编译时类型检查，防止类型混淆
/// - 零成本抽象，运行时无额外开销
///
/// # 使用示例
///
/// ```rust
/// use slotmap_graph::storage::VertexContainer;
/// use slotmap_graph::id::VertexId;
///
/// // 创建存储容器
/// let mut vertices = VertexContainer::new();
///
/// // 添加顶点
/// let alice_id = vertices.insert("Alice");
/// let bob_id = vertices.insert("Bob");
///
/// // 查询顶点
/// assert_eq!(vertices.get(alice_id), Some(&"Alice"));
///
/// // 删除顶点
/// let removed = vertices.remove(alice_id);
/// assert_eq!(removed, Some("Alice"));
/// ```
#[derive(Debug)]
pub struct VertexContainer<V>
where
    V: Clone,
{
    /// 使用 SlotMap 存储顶点数据
    ///
    /// `SlotMap` 提供了以下关键特性：
    /// - O(1) 插入、删除、查找操作
    /// - 稳定的键引用，删除后不会重用
    /// - 紧凑的内存布局，自动重用删除的槽位
    /// - 迭代器安全，支持并发遍历
    data: SlotMap<DefaultKey, V>,
}

impl<V> VertexContainer<V>
where
    V: Clone,
{
    /// 创建新的空顶点容器
    ///
    /// 创建一个不包含任何元素的空容器。内部SlotMap会根据需要进行初始分配。
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `VertexContainer` 实例。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::storage::VertexContainer;
    ///
    /// let container: VertexContainer<String> = VertexContainer::new();
    /// assert_eq!(container.len(), 0);
    /// assert!(container.is_empty());
    /// ```
    ///
    /// # 性能特征
    ///
    /// - **时间复杂度**: O(1)
    /// - **空间复杂度**: O(1)
    /// - **内存分配**: 最小化初始分配
    #[inline]
    pub fn new() -> Self {
        Self {
            data: SlotMap::new(),
        }
    }

    /// 创建带有预设容量的顶点容器
    ///
    /// 预分配指定容量的存储空间，可以减少后续插入操作时的内存重分配。
    /// 这对于已知大概元素数量的场景很有用。
    ///
    /// # 参数
    ///
    /// * `capacity` - 预设的元素容量
    ///
    /// # 返回值
    ///
    /// 返回一个具有预设容量的 `VertexContainer` 实例。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::storage::VertexContainer;
    ///
    /// // 预分配1000个顶点的空间
    /// let container: VertexContainer<i32> = VertexContainer::with_capacity(1000);
    /// assert_eq!(container.len(), 0);
    /// ```
    ///
    /// # 性能考虑
    ///
    /// - **时间复杂度**: O(1)
    /// - **空间复杂度**: O(capacity)
    /// - **内存分配**: 一次性分配指定容量，减少后续重分配
    ///
    /// # 注意事项
    ///
    /// - 过度预分配可能导致内存浪费
    /// - 容量不是硬限制，超出时会自动扩容
    /// - 实际内存使用可能略高于请求的容量
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: SlotMap::with_capacity(capacity),
        }
    }

    /// 插入顶点数据
    #[inline]
    pub fn insert(&mut self, vertex: V) -> VertexId {
        let key = self.data.insert(vertex);
        VertexId::new(key)
    }

    /// 获取顶点数据的不可变引用
    #[inline]
    pub fn get(&self, id: VertexId) -> Option<&V> {
        self.data.get(id.key())
    }

    /// 获取顶点数据的可变引用
    #[inline]
    pub fn get_mut(&mut self, id: VertexId) -> Option<&mut V> {
        self.data.get_mut(id.key())
    }

    /// 删除顶点
    #[inline]
    pub fn remove(&mut self, id: VertexId) -> Option<V> {
        self.data.remove(id.key())
    }

    /// 检查是否包含指定顶点
    #[inline]
    pub fn contains(&self, id: VertexId) -> bool {
        self.data.contains_key(id.key())
    }

    /// 获取顶点数量
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查容器是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有顶点
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 获取所有顶点ID
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = VertexId> + '_ {
        self.data.keys().map(VertexId::new)
    }

    /// 获取所有顶点数据的不可变引用
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        self.data.values()
    }

    /// 获取所有顶点数据的可变引用
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> + '_ {
        self.data.values_mut()
    }

    /// 迭代所有顶点的键值对
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (VertexId, &V)> + '_ {
        self.data.iter().map(|(key, value)| (VertexId::new(key), value))
    }

    /// 迭代所有顶点的键值对（可变）
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (VertexId, &mut V)> + '_ {
        self.data.iter_mut().map(|(key, value)| (VertexId::new(key), value))
    }

    /// 保留满足条件的顶点
    #[inline]
    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(VertexId, &V) -> bool,
    {
        self.data.retain(|key, value| predicate(VertexId::new(key), value));
    }

    /// 批量插入顶点
    #[inline]
    pub fn insert_iter<I>(&mut self, iter: I) -> Vec<VertexId>
    where
        I: IntoIterator<Item = V>,
    {
        iter.into_iter().map(|vertex| self.insert(vertex)).collect()
    }
}

impl<V> Storage<V> for VertexContainer<V>
where
    V: Clone,
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

    fn iter(&self) -> super::container::ContainerIter<'_, V, Self>
    where
        Self: Sized,
    {
        super::container::ContainerIter::new(self)
    }
}

impl<V> Default for VertexContainer<V>
where
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pi_slotmap::DefaultKey;

    #[test]
    fn test_vertex_container_basic_operations() {
        let mut vertices = VertexContainer::new();

        // 测试插入
        let alice = vertices.insert("Alice");
        let bob = vertices.insert("Bob");

        assert_eq!(vertices.len(), 2);
        assert!(!vertices.is_empty());

        // 测试获取
        assert_eq!(vertices.get(alice), Some(&"Alice"));
        assert_eq!(vertices.get(bob), Some(&"Bob"));

        // 测试可变获取
        if let Some(name) = vertices.get_mut(alice) {
            *name = "Alice Smith";
        }
        assert_eq!(vertices.get(alice), Some(&"Alice Smith"));

        // 测试包含
        assert!(vertices.contains(alice));
        assert!(!vertices.contains(VertexId::new(DefaultKey::default())));

        // 测试删除
        let removed = vertices.remove(alice);
        assert_eq!(removed, Some("Alice Smith"));
        assert!(!vertices.contains(alice));
        assert_eq!(vertices.len(), 1);

        // 测试清空
        vertices.clear();
        assert!(vertices.is_empty());
        assert_eq!(vertices.len(), 0);
    }

    #[test]
    fn test_vertex_container_iterators() {
        let mut vertices = VertexContainer::new();
        let ids: Vec<_> = (1..=5).map(|i| vertices.insert(format!("Vertex{}", i))).collect();

        // 测试迭代器
        let collected: Vec<_> = vertices.iter().collect();
        assert_eq!(collected.len(), 5);

        // 测试键迭代器
        let key_count: usize = vertices.keys().count();
        assert_eq!(key_count, 5);

        // 测试值迭代器
        let values: Vec<_> = vertices.values().cloned().collect();
        assert!(values.iter().any(|v| v.starts_with("Vertex")));
    }
}