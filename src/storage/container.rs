/**
 * # 通用容器接口
 *
 * 为存储容器提供统一的接口和通用功能。
 */

use super::{StorageKey, Storage};
use pi_slotmap::{SlotMap, DefaultKey};

/// 通用存储容器，基于 `pi_slotmap::SlotMap` 实现
#[derive(Debug)]
pub struct Container<T> {
    data: SlotMap<DefaultKey, T>,
}

impl<T> Container<T>
{
    /// 创建新的空容器
    #[inline]
    pub fn new() -> Self {
        Self {
            data: SlotMap::new(),
        }
    }

    /// 创建带有预设容量的容器
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: SlotMap::with_capacity(capacity),
        }
    }

    /// 插入元素
    #[inline]
    pub fn insert(&mut self, value: T) -> DefaultKey {
        self.data.insert(value)
    }

    /// 获取元素
    #[inline]
    pub fn get(&self, key: DefaultKey) -> Option<&T> {
        self.data.get(key)
    }

    /// 获取可变元素引用
    #[inline]
    pub fn get_mut(&mut self, key: DefaultKey) -> Option<&mut T> {
        self.data.get_mut(key)
    }

    /// 删除元素
    #[inline]
    pub fn remove(&mut self, key: DefaultKey) -> Option<T> {
        self.data.remove(key)
    }

    /// 检查是否包含指定键
    #[inline]
    pub fn contains(&self, key: DefaultKey) -> bool {
        self.data.contains_key(key)
    }

    /// 获取元素数量
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有元素
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// 获取所有键
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = DefaultKey> + '_ {
        self.data.keys()
    }

    /// 获取所有值
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.values()
    }

    /// 获取所有可变值
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.data.values_mut()
    }

    /// 迭代所有键值对
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (DefaultKey, &T)> + '_ {
        self.data.iter()
    }

    /// 迭代所有键值对（可变）
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (DefaultKey, &mut T)> + '_ {
        self.data.iter_mut()
    }
}

impl<T> Storage<T> for Container<T>
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn contains(&self, _id: impl Into<StorageKey>) -> bool {
        // 简化实现，返回 false
        false
    }

    fn iter(&self) -> ContainerIter<'_, T, Self>
    where
        Self: Sized,
    {
        ContainerIter::new(self)
    }
}

impl<T> Default for Container<T>
{
    fn default() -> Self {
        Self::new()
    }
}

/// 容器迭代器，提供统一的迭代接口
#[derive(Debug)]
pub struct ContainerIter<'a, T, C>
where
    C: 'a,
{
    container: &'a C,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T, C> ContainerIter<'a, T, C>
where
    C: 'a,
{
    /// 创建新的容器迭代器
    #[inline]
    pub fn new(container: &'a C) -> Self {
        Self {
            container,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pi_slotmap::DefaultKey;

    #[test]
    fn test_container_basic_operations() {
        let mut container: Container<i32> = Container::new();

        // 测试插入
        let key1 = container.insert(1);
        let key2 = container.insert(2);
        let key3 = container.insert(3);

        assert_eq!(container.len(), 3);
        assert!(!container.is_empty());

        // 测试获取
        assert_eq!(container.get(key1), Some(&1));
        assert_eq!(container.get(key2), Some(&2));
        assert_eq!(container.get(key3), Some(&3));

        // 测试可变获取
        if let Some(value) = container.get_mut(key2) {
            *value = 20;
        }
        assert_eq!(container.get(key2), Some(&20));

        // 测试包含
        assert!(container.contains(key1));
        assert!(!container.contains(DefaultKey::default()));

        // 测试删除
        let removed = container.remove(key1);
        assert_eq!(removed, Some(1));
        assert!(!container.contains(key1));
        assert_eq!(container.len(), 2);

        // 测试清空
        container.clear();
        assert!(container.is_empty());
        assert_eq!(container.len(), 0);
    }

    #[test]
    fn test_container_iterators() {
        let mut container: Container<i32> = Container::new();
        let keys: Vec<_> = (1..=5).map(|i| container.insert(i)).collect();

        // 测试迭代器
        let collected: Vec<_> = container.iter().collect();
        assert_eq!(collected.len(), 5);

        // 测试键迭代器
        let key_count: usize = container.keys().count();
        assert_eq!(key_count, 5);

        // 测试值迭代器
        let value_sum: i32 = container.values().sum();
        assert_eq!(value_sum, 15); // 1+2+3+4+5
    }

    #[test]
    fn test_container_with_capacity() {
        let container: Container<i32> = Container::with_capacity(100);
        assert!(container.is_empty());
    }
}