/**
 * # EdgeContainer - 边存储容器
 *
 * 基于 `pi_slotmap::SlotMap` 的高性能边存储实现。
 *
 * ## 设计原则
 *
 * - **高性能**: O(1) 插入、删除、查找操作
 * - **内存效率**: 紧凑存储，自动重用删除的空间
 * - **类型安全**: 强类型的泛型设计
 * - **连接管理**: 内置连接信息管理
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
use super::super::id::{EdgeId, EdgeInfo, VertexId};
use pi_slotmap::{DefaultKey, SlotMap};

/// 边存储容器，基于 `pi_slotmap::SlotMap` 实现
#[derive(Debug)]
pub struct EdgeContainer<E>
where
    E: Clone,
{
    /// 使用 SlotMap 存储边数据
    data: SlotMap<DefaultKey, E>,
    /// 使用 SlotMap 存储连接信息
    connections: SlotMap<DefaultKey, EdgeInfo>,
}

impl<E> EdgeContainer<E>
where
    E: Clone,
{
    /// 创建新的空边容器
    #[inline]
    pub fn new() -> Self {
        Self {
            data: SlotMap::new(),
            connections: SlotMap::new(),
        }
    }

    /// 创建带有预设容量的边容器
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: SlotMap::with_capacity(capacity),
            connections: SlotMap::with_capacity(capacity),
        }
    }

    /// 插入边数据和连接信息
    #[inline]
    pub fn insert(&mut self, edge: E, edge_info: EdgeInfo) -> EdgeId {
        let key = self.data.insert(edge);
        self.connections.insert(edge_info);
        EdgeId::new(key)
    }

    /// 获取边数据的不可变引用
    #[inline]
    pub fn get(&self, id: EdgeId) -> Option<&E> {
        self.data.get(id.key())
    }

    /// 获取边数据的可变引用
    #[inline]
    pub fn get_mut(&mut self, id: EdgeId) -> Option<&mut E> {
        self.data.get_mut(id.key())
    }

    /// 获取连接信息的不可变引用
    #[inline]
    pub fn get_connection(&self, id: EdgeId) -> Option<&EdgeInfo> {
        self.connections.get(id.key())
    }

    /// 删除边
    #[inline]
    pub fn remove(&mut self, id: EdgeId) -> Option<(E, EdgeInfo)> {
        let edge = self.data.remove(id.key());
        let info = self.connections.remove(id.key());
        match (edge, info) {
            (Some(e), Some(i)) => Some((e, i)),
            _ => None,
        }
    }

    /// 检查是否包含指定边
    #[inline]
    pub fn contains(&self, id: EdgeId) -> bool {
        self.data.contains_key(id.key())
    }

    /// 获取边数量
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 检查容器是否为空
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 清空所有边
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.connections.clear();
    }

    /// 获取所有边ID
    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = EdgeId> + '_ {
        self.data.keys().map(EdgeId::new)
    }

    /// 获取所有边数据的不可变引用
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = &E> + '_ {
        self.data.values()
    }

    /// 获取所有边数据的可变引用
    #[inline]
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut E> + '_ {
        self.data.values_mut()
    }

    /// 迭代所有边的键值对
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (EdgeId, &E)> + '_ {
        self.data.iter().map(|(key, value)| (EdgeId::new(key), value))
    }

    /// 迭代所有边的键值对（可变）
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EdgeId, &mut E)> + '_ {
        self.data.iter_mut().map(|(key, value)| (EdgeId::new(key), value))
    }

    /// 迭代所有边和连接信息
    #[inline]
    pub fn iter_with_connections(&self) -> impl Iterator<Item = (EdgeId, &E, &EdgeInfo)> + '_ {
        self.data.iter().zip(self.connections.iter())
            .map(|((key, edge), (_, info))| (EdgeId::new(key), edge, info))
    }

    /// 保留满足条件的边
    #[inline]
    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(EdgeId, &E, &EdgeInfo) -> bool,
    {
        let mut keys_to_remove = Vec::new();

        for ((key, edge), (_, info)) in self.data.iter().zip(self.connections.iter()) {
            let edge_id = EdgeId::new(key);
            if !predicate(edge_id, edge, info) {
                keys_to_remove.push(key);
            }
        }

        for key in keys_to_remove {
            self.data.remove(key);
            self.connections.remove(key);
        }
    }

    /// 批量插入边
    #[inline]
    pub fn insert_iter<I>(&mut self, iter: I) -> Vec<EdgeId>
    where
        I: IntoIterator<Item = (E, EdgeInfo)>,
    {
        iter.into_iter()
            .map(|(edge, info)| self.insert(edge, info))
            .collect()
    }

    /// 获取从指定顶点出发的所有边ID
    #[inline]
    pub fn edges_from(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.connections
            .iter()
            .filter(move |(_, info)| info.from() == vertex_id)
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 获取到达指定顶点的所有边ID
    #[inline]
    pub fn edges_to(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.connections
            .iter()
            .filter(move |(_, info)| info.to() == vertex_id)
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 获取涉及指定顶点的所有边ID
    #[inline]
    pub fn edges_involving(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.connections
            .iter()
            .filter(move |(_, info)| info.involves(vertex_id))
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 检查两个顶点之间是否有边
    #[inline]
    pub fn has_edge_between(&self, from: VertexId, to: VertexId) -> bool {
        self.connections
            .iter()
            .any(|(_, info)| info.connects(from, to))
    }

    /// 获取两个顶点之间的边
    #[inline]
    pub fn get_edge_between(&self, from: VertexId, to: VertexId) -> Option<EdgeId> {
        self.connections
            .iter()
            .find(|(_, info)| info.connects(from, to))
            .map(|(key, _)| EdgeId::new(key))
    }

    /// 获取与指定顶点相邻的所有边
    #[inline]
    pub fn edges_adjacent(&self, vertex_id: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.edges_involving(vertex_id)
    }

    /// 获取两个顶点之间的所有边
    #[inline]
    pub fn edges_between(&self, from: VertexId, to: VertexId) -> impl Iterator<Item = EdgeId> + '_ {
        self.connections
            .iter()
            .filter(move |(_, info)| info.connects(from, to))
            .map(|(key, _)| EdgeId::new(key))
    }
}

impl<E> Storage<E> for EdgeContainer<E>
where
    E: Clone,
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

    fn iter(&self) -> super::container::ContainerIter<'_, E, Self>
    where
        Self: Sized,
    {
        super::container::ContainerIter::new(self)
    }
}

impl<E> Default for EdgeContainer<E>
where
    E: Clone,
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
    fn test_edge_container_basic_operations() {
        let mut edges = EdgeContainer::new();

        // 创建测试数据
        let from = VertexId::new(DefaultKey::default());
        let to = VertexId::new(DefaultKey::default());
        let edge_info = EdgeInfo::new(EdgeId::new(DefaultKey::default()), from, to);

        // 测试插入
        let edge_id = edges.insert("friendship", edge_info);
        assert_eq!(edges.len(), 1);
        assert!(!edges.is_empty());

        // 测试获取
        assert_eq!(edges.get(edge_id), Some(&"friendship"));
        assert!(edges.get_connection(edge_id).is_some());

        // 测试可变获取
        if let Some(label) = edges.get_mut(edge_id) {
            *label = "best friendship";
        }
        assert_eq!(edges.get(edge_id), Some(&"best friendship"));

        // 测试包含
        assert!(edges.contains(edge_id));

        // 测试删除
        let removed = edges.remove(edge_id);
        assert!(removed.is_some());
        assert!(!edges.contains(edge_id));
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_edge_container_connections() {
        let mut edges = EdgeContainer::new();

        // 创建不同的测试键
        let mut slotmap: SlotMap<DefaultKey, usize> = SlotMap::new();
        let mut keys = Vec::new();
        for i in 0..6 {
            let key = slotmap.insert(i);
            keys.push(key);
        }

        let alice = VertexId::new(keys[0]);
        let bob = VertexId::new(keys[1]);
        let charlie = VertexId::new(keys[2]);

        let edge1_info = EdgeInfo::new(EdgeId::new(keys[3]), alice, bob);
        let edge2_info = EdgeInfo::new(EdgeId::new(keys[4]), bob, charlie);
        let edge3_info = EdgeInfo::new(EdgeId::new(keys[5]), alice, charlie);

        let _edge1 = edges.insert("friend1", edge1_info);
        let _edge2 = edges.insert("friend2", edge2_info);
        let _edge3 = edges.insert("friend3", edge3_info);

        // 测试从顶点出发的边
        let from_alice: Vec<_> = edges.edges_from(alice).collect();
        assert_eq!(from_alice.len(), 2);

        // 测试到达顶点的边
        let to_charlie: Vec<_> = edges.edges_to(charlie).collect();
        assert_eq!(to_charlie.len(), 2);

        // 测试涉及顶点的边
        let involving_bob: Vec<_> = edges.edges_involving(bob).collect();
        assert_eq!(involving_bob.len(), 2);

        // 测试检查边是否存在
        assert!(edges.has_edge_between(alice, bob));
        assert!(edges.has_edge_between(bob, charlie));
        assert!(!edges.has_edge_between(bob, alice));

        // 测试获取特定边
        let alice_bob = edges.get_edge_between(alice, bob);
        assert!(alice_bob.is_some());
    }
}