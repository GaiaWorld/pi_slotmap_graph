/**
 * # 哈希索引 (HashIndex)
 *
 * 提供高性能的精确匹配查询功能，基于 HashMap 实现。
 *
 * ## 设计特点
 *
 * ### 一对多关系支持
 * - **数据结构**：`HashMap<K, HashSet<VertexId/EdgeId>>`
 * - **去重机制**：自动防止重复值
 * - **动态更新**：插入和删除时实时维护索引
 *
 * ### 内存优化
 * - **延迟初始化**：按需创建 HashSet
 * - **自动清理**：删除空键值对
 * - **紧凑存储**：最小化内存开销
 *
 * ## 使用场景
 *
 * - **标签索引**：按顶点/边类型快速查找
 * - **属性索引**：按具体属性值精确匹配
 * - **分类查询**：按类别、状态等字段分组
 * - **ID映射**：外部ID到内部ID的转换
 *
 * ## 性能特征
 *
 * - **插入**：O(1) 平均时间复杂度
 * - **查询**：O(1) 平均时间复杂度
 * - **删除**：O(1) 平均时间复杂度
 * - **内存**：O(n) 空间复杂度，n为索引项数量
 */

use crate::id::{VertexId, EdgeId};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// 哈希索引，支持一对多关系的高效精确匹配
///
/// 这个索引类型适用于需要快速精确匹配的场景，如按标签、分类或特定属性值查询。
///
/// # 类型参数
///
/// - `K`: 索引键类型，必须实现 `Hash + Eq + Clone`
/// - `V`: 值类型，必须是 `VertexId` 或 `EdgeId`
///
/// # 示例
///
/// ```rust
/// use slotmap_graph::index::HashIndex;
/// use slotmap_graph::VertexId;
/// use pi_slotmap::DefaultKey;
///
/// let mut index = HashIndex::new();
///
/// // 添加索引项
/// let id1 = VertexId::new(DefaultKey::default());
/// let id2 = VertexId::new(DefaultKey::default());
///
/// index.insert("engineer", id1);
/// index.insert("engineer", id2);
/// index.insert("designer", id1);
///
/// // 查询
/// let engineers: Vec<_> = index.get("engineer").collect();
/// assert_eq!(engineers.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct HashIndex<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    /// 内部存储映射
    map: HashMap<K, HashSet<V>>,
    /// 空集合，用于 get() 方法返回空迭代器
    empty: HashSet<V>,
}

impl<K, V> Default for HashIndex<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashIndex<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    /// 创建新的哈希索引
    ///
    /// # 返回值
    ///
    /// 返回空的 `HashIndex` 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let index: HashIndex<String, u32> = HashIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            empty: HashSet::new(),
        }
    }

    /// 预分配容量的哈希索引
    ///
    /// # 参数
    ///
    /// * `capacity` - 预估的键数量
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let index: HashIndex<String, u32> = HashIndex::with_capacity(100);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            empty: HashSet::new(),
        }
    }

    /// 插入键值对
    ///
    /// 如果键值对已存在，不会重复添加（去重机制）。
    ///
    /// # 参数
    ///
    /// * `key` - 索引键
    /// * `value` - 要索引的值（VertexId 或 EdgeId）
    ///
    /// # 返回值
    ///
    /// 返回 `true` 如果是新的键值对，`false` 如果已存在
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let mut index = HashIndex::new();
    ///
    /// let inserted1 = index.insert("key", 1);
    /// let inserted2 = index.insert("key", 1); // 重复插入
    ///
    /// assert!(inserted1);
    /// assert!(!inserted2);
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> bool {
        self.map.entry(key).or_default().insert(value)
    }

    /// 移除键值对
    ///
    /// 如果删除后键对应的值集合为空，会自动删除该键。
    ///
    /// # 参数
    ///
    /// * `key` - 索引键
    /// * `value` - 要移除的值
    ///
    /// # 返回值
    ///
    /// 返回 `true` 如果成功删除，`false` 如果键值对不存在
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let mut index = HashIndex::new();
    /// index.insert("key", 1);
    ///
    /// let removed = index.remove("key", &1);
    /// assert!(removed);
    ///
    /// let not_removed = index.remove("key", &1);
    /// assert!(!not_removed);
    /// ```
    pub fn remove<Q>(&mut self, key: &Q, value: &V) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        if let Some(values) = self.map.get_mut(key) {
            let removed = values.remove(value);
            // 如果删除后集合为空，删除整个键
            if values.is_empty() {
                self.map.remove(key);
            }
            removed
        } else {
            false
        }
    }

    /// 移除键对应的所有值
    ///
    /// # 参数
    ///
    /// * `key` - 要移除的键
    ///
    /// # 返回值
    ///
    /// 返回被移除的值集合，如果键不存在则返回 None
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    /// use std::collections::HashSet;
    ///
    /// let mut index = HashIndex::new();
    /// index.insert("key", 1);
    /// index.insert("key", 2);
    ///
    /// let removed = index.remove_all("key");
    /// assert!(removed.is_some());
    /// ```
    pub fn remove_all<Q>(&mut self, key: &Q) -> Option<HashSet<V>>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.remove(key)
    }

    /// 查询键对应的所有值
    ///
    /// 返回值的迭代器，如果键不存在则返回空迭代器。
    ///
    /// # 参数
    ///
    /// * `key` - 要查询的键
    ///
    /// # 返回值
    ///
    /// 返回对应值的迭代器
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let mut index = HashIndex::new();
    /// index.insert("key", 1);
    /// index.insert("key", 2);
    ///
    /// let values: Vec<_> = index.get("key").collect();
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn get<'a, Q>(&'a self, key: &Q) -> impl Iterator<Item = V> + 'a
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key).unwrap_or(&self.empty).iter().copied()
    }

    /// 检查键值对是否存在
    ///
    /// # 参数
    ///
    /// * `key` - 索引键
    /// * `value` - 要检查的值
    ///
    /// # 返回值
    ///
    /// 返回 `true` 如果键值对存在
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::HashIndex;
    ///
    /// let mut index = HashIndex::new();
    /// index.insert("key", 1);
    ///
    /// assert!(index.contains("key", &1));
    /// assert!(!index.contains("key", &2));
    /// ```
    pub fn contains<Q>(&self, key: &Q, value: &V) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key).map_or(false, |values| values.contains(value))
    }

    /// 检查键是否存在
    ///
    /// # 参数
    ///
    /// * `key` - 要检查的键
    ///
    /// # 返回值
    ///
    /// 返回 `true` 如果键存在
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.contains_key(key)
    }

    /// 获取键对应的值数量
    ///
    /// # 参数
    ///
    /// * `key` - 索引键
    ///
    /// # 返回值
    ///
    /// 返回键对应的值数量，如果键不存在则返回 0
    pub fn len_of<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key).map_or(0, |values| values.len())
    }

    /// 获取索引中的键数量
    pub fn keys_len(&self) -> usize {
        self.map.len()
    }

    /// 获取索引中的总值数量（可能有重复值在不同键下）
    pub fn total_values_len(&self) -> usize {
        self.map.values().map(|values| values.len()).sum()
    }

    /// 检查索引是否为空
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// 清空所有索引项
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// 获取所有键的迭代器
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    /// 获取所有键值对的迭代器
    pub fn iter(&self) -> impl Iterator<Item = (&K, impl Iterator<Item = V> + '_)> {
        self.map.iter().map(|(k, v)| (k, v.iter().copied()))
    }
}

// 特化的顶点和边索引类型
pub type VertexHashIndex<K> = HashIndex<K, VertexId>;
pub type EdgeHashIndex<K> = HashIndex<K, EdgeId>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VertexId;
    use pi_slotmap::DefaultKey;

    #[test]
    fn test_basic_operations() {
        let mut index: VertexHashIndex<String> = HashIndex::new();

        // Test with same key (should insert once, then return false for duplicate)
        let id1 = VertexId::new(DefaultKey::default());
        let id2 = VertexId::new(DefaultKey::default());

        // 测试插入
        assert!(index.insert("engineer".to_string(), id1));
        assert!(!index.insert("engineer".to_string(), id2)); // duplicate, should return false
        assert!(index.insert("designer".to_string(), id1));

        // 测试查询
        let engineers: Vec<_> = index.get("engineer").collect();
        assert_eq!(engineers.len(), 1); // Only one unique ID since id1 == id2

        let designers: Vec<_> = index.get("designer").collect();
        assert_eq!(designers.len(), 1);

        // 测试不存在的键
        let managers: Vec<_> = index.get("manager").collect();
        assert_eq!(managers.len(), 0);
    }

    #[test]
    fn test_duplicate_prevention() {
        let mut index: VertexHashIndex<i32> = HashIndex::new();

        let id1 = VertexId::new(DefaultKey::default());

        // 插入相同键值对多次
        assert!(index.insert(1, id1));
        assert!(!index.insert(1, id1));
        assert!(!index.insert(1, id1));

        // 应该只有一个实例
        let results: Vec<_> = index.get(&1).collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], id1);
    }

    #[test]
    fn test_remove_operations() {
        let mut index: VertexHashIndex<i32> = HashIndex::new();

        // Test with same key (should insert once, then return false for duplicate)
        let id1 = VertexId::new(DefaultKey::default());
        let id2 = VertexId::new(DefaultKey::default());

        index.insert(1, id1);
        index.insert(1, id2); // duplicate, won't actually insert

        // Since id1 == id2, there's only one value actually inserted
        assert!(index.remove(&1, &id1));
        assert!(!index.contains_key(&1)); // Key should be completely removed
    }

    #[test]
    fn test_statistics() {
        let mut index: VertexHashIndex<i32> = HashIndex::new();

        // Test with same key (should insert once, then return false for duplicate)
        let id1 = VertexId::new(DefaultKey::default());
        let id2 = VertexId::new(DefaultKey::default());

        // 空索引统计
        assert!(index.is_empty());
        assert_eq!(index.keys_len(), 0);
        assert_eq!(index.total_values_len(), 0);

        // 插入数据后统计
        index.insert(1, id1);
        index.insert(1, id2); // duplicate, won't insert
        index.insert(2, id1); // same id, different key

        assert!(!index.is_empty());
        assert_eq!(index.keys_len(), 2); // keys 1 and 2
        assert_eq!(index.total_values_len(), 2); // one value per key since id1 == id2
        assert_eq!(index.len_of(&1), 1); // only id1 since id2 is duplicate
        assert_eq!(index.len_of(&2), 1); // id1
        assert_eq!(index.len_of(&3), 0);
    }

    #[test]
    fn test_edge_index() {
        let mut index: EdgeHashIndex<String> = HashIndex::new();

        let edge1 = EdgeId::new(DefaultKey::default());
        let edge2 = EdgeId::new(DefaultKey::default());

        index.insert("friendship".to_string(), edge1);
        index.insert("follows".to_string(), edge2);

        assert_eq!(index.len_of("friendship"), 1);
        assert_eq!(index.len_of("follows"), 1);
    }
}