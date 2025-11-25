/**
 * # 范围索引 (RangeIndex)
 *
 * 提供高效的有序范围查询功能，基于 BTreeMap 实现。
 *
 * ## 设计特点
 *
 * ### 有序存储
 * - **数据结构**：`BTreeMap<K, HashSet<VertexId/EdgeId>>`
 * - **自然排序**：键自动按序存储
 * - **范围查询**：支持复杂范围条件的快速查询
 *
 * ### 高效范围操作
 * - **单点查询**：O(log n) 时间复杂度
 * - **范围查询**：O(log n + k) 时间复杂度，k为结果数量
 * - **边界支持**：包含/排除上下界的完整支持
 *
 * ## 使用场景
 *
 * - **时间范围**：按时间戳查询事件
 * - **数值范围**：按年龄、价格、分数等查询
 * - **字母范围**：按名称首字母查询
 * - **版本范围**：按版本号查询
 * - **优先级查询**：按优先级范围过滤
 *
 * ## 性能特征
 *
 * - **插入**：O(log n) 平均时间复杂度
 * - **单点查询**：O(log n) 平均时间复杂度
 * - **范围查询**：O(log n + k) 时间复杂度
 * - **内存**：O(n) 空间复杂度，比哈希索引稍高
 */

use crate::id::{VertexId, EdgeId};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Bound, RangeBounds};

/// 范围索引，支持有序键的范围查询
///
/// 这个索引类型适用于需要范围查询的场景，如时间范围、数值范围查询等。
///
/// # 类型参数
///
/// - `K`: 索引键类型，必须实现 `Ord`（可排序）
/// - `V`: 值类型，必须是 `VertexId` 或 `EdgeId`
///
/// # 示例
///
/// ```rust
/// use slotmap_graph::index::RangeIndex;
/// use slotmap_graph::VertexId;
/// use pi_slotmap::DefaultKey;
/// use std::ops::Bound;
///
/// let mut index = RangeIndex::new();
///
/// // 添加索引项
/// let id1 = VertexId::new(DefaultKey::default());
/// let id2 = VertexId::new(DefaultKey::default());
///
/// index.insert(25, id1);  // 年龄25
/// index.insert(30, id2);  // 年龄30
///
/// // 范围查询
/// let young_people: Vec<_> = index.range(20..30).collect();
/// assert_eq!(young_people.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct RangeIndex<K, V>
where
    K: Ord + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    /// 内部有序映射
    map: BTreeMap<K, HashSet<V>>,
    /// 空集合，用于 get() 方法返回空迭代器
    empty: HashSet<V>,
}

impl<K, V> Default for RangeIndex<K, V>
where
    K: Ord + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> RangeIndex<K, V>
where
    K: Ord + Clone + Debug,
    V: Hash + Eq + Clone + Copy + Debug,
{
    /// 创建新的范围索引
    ///
    /// # 返回值
    ///
    /// 返回空的 `RangeIndex` 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let index: RangeIndex<u32, u32> = RangeIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            empty: HashSet::new(),
        }
    }

    /// 插入键值对
    ///
    /// 如果键值对已存在，不会重复添加（去重机制）。
    ///
    /// # 参数
    ///
    /// * `key` - 索引键（必须可排序）
    /// * `value` - 要索引的值（VertexId 或 EdgeId）
    ///
    /// # 返回值
    ///
    /// 返回 `true` 如果是新的键值对，`false` 如果已存在
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    ///
    /// let inserted1 = index.insert(25, 1);
    /// let inserted2 = index.insert(25, 1); // 重复插入
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
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    /// index.insert(25, 1);
    ///
    /// let removed = index.remove(&25, &1);
    /// assert!(removed);
    /// ```
    pub fn remove<Q>(&mut self, key: &Q, value: &V) -> bool
    where
        K: Borrow<Q> + Ord,
        Q: ?Sized + Ord,
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
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    /// index.insert(25, 1);
    /// index.insert(25, 2);
    ///
    /// let values: Vec<_> = index.get(&25).collect();
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn get<'a, Q>(&'a self, key: &Q) -> impl Iterator<Item = V> + 'a
    where
        K: Borrow<Q>,
        Q: ?Sized + Ord,
    {
        self.map.get(key).unwrap_or(&self.empty).iter().copied()
    }

    /// 范围查询
    ///
    /// 支持所有类型的范围边界：包含、排除、无界。
    ///
    /// # 参数
    ///
    /// * `range` - 查询范围，实现了 `RangeBounds` trait
    ///
    /// # 返回值
    ///
    /// 返回范围内所有值的迭代器
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    /// use std::ops::Bound;
    ///
    /// let mut index = RangeIndex::new();
    /// for i in 20..30 {
    ///     index.insert(i, i);
    /// }
    ///
    /// // 包含范围
    /// let inclusive: Vec<_> = index.range(22..=28).collect();
    /// assert_eq!(inclusive.len(), 7);
    ///
    /// // 排除范围
    /// let exclusive: Vec<_> = index.range(22..28).collect();
    /// assert_eq!(exclusive.len(), 6);
    ///
    /// // 单边界
    /// let from: Vec<_> = index.range(25..).collect();
    /// let to: Vec<_> = index.range(..=25).collect();
    /// ```
    pub fn range<R>(&self, range: R) -> impl Iterator<Item = V> + '_
    where
        R: RangeBounds<K>,
    {
        self.map.range(range).flat_map(|(_, v)| v.iter().copied())
    }

    /// 获取大于等于指定键的所有值
    ///
    /// # 参数
    ///
    /// * `key` - 起始键（包含）
    ///
    /// # 返回值
    ///
    /// 返回大于等于指定键的所有值的迭代器
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    /// for i in 10..20 {
    ///     index.insert(i, i);
    /// }
    ///
    /// let from_15: Vec<_> = index.from(&15).collect();
    /// assert_eq!(from_15.len(), 5); // 15, 16, 17, 18, 19
    /// ```
    pub fn from(&self, key: &K) -> impl Iterator<Item = V> + '_ {
        self.map.range(key..).flat_map(|(_, v)| v.iter().copied())
    }

    /// 获取小于等于指定键的所有值
    ///
    /// # 参数
    ///
    /// * `key` - 结束键（包含）
    ///
    /// # 返回值
    ///
    /// 返回小于等于指定键的所有值的迭代器
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    /// for i in 10..20 {
    ///     index.insert(i, i);
    /// }
    ///
    /// let to_15: Vec<_> = index.to(&15).collect();
    /// assert_eq!(to_15.len(), 6); // 10, 11, 12, 13, 14, 15
    /// ```
    pub fn to(&self, key: &K) -> impl Iterator<Item = V> + '_ {
        self.map.range(..=key).flat_map(|(_, v)| v.iter().copied())
    }

    /// 获取大于指定键的所有值
    ///
    /// # 参数
    ///
    /// * `key` - 起始键（排除）
    ///
    /// # 返回值
    ///
    /// 返回大于指定键的所有值的迭代器
    pub fn after(&self, key: &K) -> impl Iterator<Item = V> + '_ {
        let start = Bound::Excluded(key);
        self.map.range((start, Bound::Unbounded)).flat_map(|(_, v)| v.iter().copied())
    }

    /// 获取小于指定键的所有值
    ///
    /// # 参数
    ///
    /// * `key` - 结束键（排除）
    ///
    /// # 返回值
    ///
    /// 返回小于指定键的所有值的迭代器
    pub fn before(&self, key: &K) -> impl Iterator<Item = V> + '_ {
        let end = Bound::Excluded(key);
        self.map.range((Bound::Unbounded, end)).flat_map(|(_, v)| v.iter().copied())
    }

    /// 获取键的范围统计信息
    ///
    /// # 返回值
    ///
    /// 返回 `(min_key, max_key)` 如果索引不为空，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::RangeIndex;
    ///
    /// let mut index = RangeIndex::new();
    /// index.insert(10, 1);
    /// index.insert(20, 2);
    /// index.insert(15, 3);
    ///
    /// let (min, max) = index.range_bounds().unwrap();
    /// assert_eq!(*min, 10);
    /// assert_eq!(*max, 20);
    /// ```
    pub fn range_bounds(&self) -> Option<(&K, &K)> {
        match (self.map.keys().next(), self.map.keys().next_back()) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }

    /// 获取索引中的键数量
    pub fn keys_len(&self) -> usize {
        self.map.len()
    }

    /// 获取索引中的总值数量
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

    /// 获取所有键的迭代器（按键排序）
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }

    /// 获取所有键值对的迭代器（按键排序）
    pub fn iter(&self) -> impl Iterator<Item = (&K, impl Iterator<Item = V> + '_)> {
        self.map.iter().map(|(k, v)| (k, v.iter().copied()))
    }
}

// 特化的顶点和边索引类型
pub type VertexRangeIndex<K> = RangeIndex<K, VertexId>;
pub type EdgeRangeIndex<K> = RangeIndex<K, EdgeId>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VertexId;
    use pi_slotmap::DefaultKey;
  
    #[test]
    fn test_basic_range_operations() {
        let mut index: VertexRangeIndex<u32> = RangeIndex::new();

        let _id1 = VertexId::new(DefaultKey::default());
        let _id2 = VertexId::new(DefaultKey::default());

        // 插入测试数据
        for i in 10..20 {
            let id = VertexId::new(DefaultKey::default());
            index.insert(i, id);
        }

        // 测试单点查询
        let single: Vec<_> = index.get(&15).collect();
        assert_eq!(single.len(), 1);

        // 测试范围查询
        let range_12_17: Vec<_> = index.range(12..=17).collect();
        assert_eq!(range_12_17.len(), 6); // 12, 13, 14, 15, 16, 17

        let exclusive_range: Vec<_> = index.range(13..17).collect();
        assert_eq!(exclusive_range.len(), 4); // 13, 14, 15, 16 (all the same ID)
    }

    #[test]
    fn test_boundary_operations() {
        let mut index: VertexRangeIndex<u32> = RangeIndex::new();

        // 插入测试数据
        for i in 20..30 {
            let id = VertexId::new(DefaultKey::default());
            index.insert(i, id);
        }

        // 测试 from 操作
        let from_25: Vec<_> = index.from(&25).collect();
        assert_eq!(from_25.len(), 5); // 25, 26, 27, 28, 29

        // 测试 to 操作
        let to_25: Vec<_> = index.to(&25).collect();
        assert_eq!(to_25.len(), 6); // 20, 21, 22, 23, 24, 25

        // 测试 after 操作
        let after_25: Vec<_> = index.after(&25).collect();
        assert_eq!(after_25.len(), 4); // 26, 27, 28, 29

        // 测试 before 操作
        let before_25: Vec<_> = index.before(&25).collect();
        assert_eq!(before_25.len(), 5); // 20, 21, 22, 23, 24
    }

    #[test]
    fn test_string_keys() {
        let mut index: VertexRangeIndex<String> = RangeIndex::new();

        let id1 = VertexId::new(DefaultKey::default());
        let id2 = VertexId::new(DefaultKey::default());

        index.insert("alice".to_string(), id1);
        index.insert("bob".to_string(), id2);
        index.insert("charlie".to_string(), id1);

        // 按字母顺序范围查询
        let a_to_b: Vec<_> = index.range("a".to_string().."c".to_string()).collect();
        assert_eq!(a_to_b.len(), 2); // alice, bob

        let from_b: Vec<_> = index.from(&"b".to_string()).collect();
        assert_eq!(from_b.len(), 2); // bob, charlie
    }

    #[test]
    fn test_duplicate_prevention() {
        let mut index: VertexRangeIndex<i32> = RangeIndex::new();

        let id1 = VertexId::new(DefaultKey::default());

        // 重复插入相同的键值对
        assert!(index.insert(25, id1));
        assert!(!index.insert(25, id1));
        assert!(!index.insert(25, id1));

        // 应该只有一个实例
        let results: Vec<_> = index.get(&25).collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], id1);
    }

    #[test]
    fn test_range_bounds() {
        let mut index: VertexRangeIndex<u32> = RangeIndex::new();

        // 空索引
        assert!(index.range_bounds().is_none());

        // 插入数据
        index.insert(30, VertexId::new(DefaultKey::default()));
        index.insert(10, VertexId::new(DefaultKey::default()));
        index.insert(20, VertexId::new(DefaultKey::default()));

        let (min, max) = index.range_bounds().unwrap();
        assert_eq!(*min, 10);
        assert_eq!(*max, 30);
    }

    #[test]
    fn test_edge_index() {
        let mut index: EdgeRangeIndex<i64> = RangeIndex::new();

        let edge1 = EdgeId::new(DefaultKey::default());
        let edge2 = EdgeId::new(DefaultKey::default());

        index.insert(1000, edge1);
        index.insert(2000, edge2);

        let expensive_edges: Vec<_> = index.range(1500..).collect();
        assert_eq!(expensive_edges.len(), 1);
    }

    #[test]
    fn test_complex_boundaries() {
        let mut index: VertexRangeIndex<u32> = RangeIndex::new();

        // 插入测试数据
        for i in 0..100 {
            if i % 10 == 0 {
                let id = VertexId::new(DefaultKey::default());
                index.insert(i, id);
            }
        }

        // 测试各种边界组合
        let unbounded: Vec<_> = index.range(..).collect();
        assert_eq!(unbounded.len(), 10);

        let start_unbounded: Vec<_> = index.range(..=50).collect();
        assert_eq!(start_unbounded.len(), 6); // 0, 10, 20, 30, 40, 50

        let end_unbounded: Vec<_> = index.range(20..).collect();
        assert_eq!(end_unbounded.len(), 8); // 20, 30, 40, 50, 60, 70, 80, 90
    }
}