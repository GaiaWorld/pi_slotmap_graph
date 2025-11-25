/**
 * # 索引元数据 (Index Metadata)
 *
 * 定义索引的元数据结构和管理接口，支持动态索引创建和管理。
 *
 * ## 设计理念
 *
 * ### 类型安全的索引定义
 * - **编译时检查**：索引键类型的编译时验证
 * - **运行时分发**：基于类型信息的动态索引选择
 * - **泛型支持**：支持任意可索引的数据类型
 *
 * ### 灵活的索引策略
 * - **自动类型推断**：根据键类型自动选择索引实现
 * - **自定义提取函数**：支持复杂对象的属性提取
 * - **索引组合**：支持多字段的复合索引
 *
 * ## 核心组件
 *
 * - **IndexType**: 索引类型枚举（Hash, Range, FullText等）
 * - IndexDefinition: 索引定义，包含提取函数和类型信息
 * - IndexMetadata: 运行时索引元数据管理
 */

use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::marker::PhantomData;

use graph_api_lib::IndexType;



/// 索引值类型枚举
///
/// 统一表示不同类型的索引键值，用于运行时类型擦除和查询参数传递。
#[derive(Debug, Clone)]
pub enum IndexValue {
    String(String),
    U32(u32),
    U64(u64),
    U128(u128),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Bool(bool),
    // 未来可扩展：Bytes, DateTime, Custom等
}

impl IndexValue {
    /// 获取值的类型标识符
    pub fn type_id(&self) -> TypeId {
        match self {
            IndexValue::String(_) => TypeId::of::<String>(),
            IndexValue::U32(_) => TypeId::of::<u32>(),
            IndexValue::U64(_) => TypeId::of::<u64>(),
            IndexValue::U128(_) => TypeId::of::<u128>(),
            IndexValue::I32(_) => TypeId::of::<i32>(),
            IndexValue::I64(_) => TypeId::of::<i64>(),
            IndexValue::I128(_) => TypeId::of::<i128>(),
            IndexValue::F32(_) => TypeId::of::<f32>(),
            IndexValue::F64(_) => TypeId::of::<f64>(),
            IndexValue::Bool(_) => TypeId::of::<bool>(),
        }
    }

    /// 尝试转换为指定类型
    ///
    /// # 类型参数
    ///
    /// * `T` - 目标类型
    ///
    /// # 返回值
    ///
    /// 返回转换后的值，如果类型不匹配则返回 None
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        match self {
            IndexValue::String(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::U32(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::U64(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::U128(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::I32(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::I64(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::I128(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::F32(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::F64(v) => (v as &dyn Any).downcast_ref::<T>(),
            IndexValue::Bool(v) => (v as &dyn Any).downcast_ref::<T>(),
        }
    }
}

// 为常见类型实现从 IndexValue 的转换
impl From<String> for IndexValue {
    fn from(value: String) -> Self {
        IndexValue::String(value)
    }
}

impl From<&str> for IndexValue {
    fn from(value: &str) -> Self {
        IndexValue::String(value.to_string())
    }
}

impl From<u32> for IndexValue {
    fn from(value: u32) -> Self {
        IndexValue::U32(value)
    }
}

impl From<u64> for IndexValue {
    fn from(value: u64) -> Self {
        IndexValue::U64(value)
    }
}

impl From<i32> for IndexValue {
    fn from(value: i32) -> Self {
        IndexValue::I32(value)
    }
}

impl From<i64> for IndexValue {
    fn from(value: i64) -> Self {
        IndexValue::I64(value)
    }
}

impl From<bool> for IndexValue {
    fn from(value: bool) -> Self {
        IndexValue::Bool(value)
    }
}

/// 索引定义特征
///
/// 定义了如何从数据中提取索引键的接口。
pub trait IndexDefinition<T> {
    /// 索引键类型
    type Key: Clone + Debug + Send + Sync + 'static;

    /// 从数据中提取索引键
    fn extract(&self, data: &T) -> Self::Key;

    /// 索引类型
    fn index_type(&self) -> IndexType;

    /// 索引名称
    fn name(&self) -> &str;
}

/// 泛型索引定义实现
///
/// 使用闭包或函数指针从数据中提取索引键。
pub struct GenericIndexDefinition<T, K, F> {
    name: String,
    index_type: IndexType,
    extract_fn: F,
    _phantom: PhantomData<(T, K)>,
}

impl<T, K, F> GenericIndexDefinition<T, K, F>
where
    K: Clone + Debug + Send + Sync + 'static,
    F: Fn(&T) -> K,
{
    /// 创建新的索引定义
    ///
    /// # 参数
    ///
    /// * `name` - 索引名称
    /// * `index_type` - 索引类型
    /// * `extract_fn` - 索引键提取函数
    ///
    /// # 示例
    ///
    /// ```rust
    /// use slotmap_graph::index::{GenericIndexDefinition, IndexType};
    ///
    /// // 按年龄索引
    /// let age_index = GenericIndexDefinition::new(
    ///     "age",
    ///     IndexType::Range,
    ///     |person: &(String, u32)| person.1
    /// );
    /// ```
    pub fn new(name: String, index_type: IndexType, extract_fn: F) -> Self {
        Self {
            name,
            index_type,
            extract_fn,
            _phantom: PhantomData,
        }
    }
}

impl<T, K, F> IndexDefinition<T> for GenericIndexDefinition<T, K, F>
where
    K: Clone + Debug + Send + Sync + 'static,
    F: Fn(&T) -> K,
{
    type Key = K;

    fn extract(&self, data: &T) -> Self::Key {
        (self.extract_fn)(data)
    }

    fn index_type(&self) -> IndexType {
        self.index_type
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 索引元数据
///
/// 存储索引的运行时信息，用于索引管理和查询优化。
#[derive(Debug)]
pub struct IndexMetadata {
    /// 索引名称
    pub name: String,
    /// 索引类型
    pub index_type: IndexType,
    /// 键类型标识符
    pub key_type_id: TypeId,
    /// 索引统计信息
    pub stats: IndexStats,
    /// 是否已初始化
    pub initialized: bool,
}

/// 索引统计信息
#[derive(Debug, Clone, Default)]
pub struct IndexStats {
    /// 索引项数量
    pub entries: usize,
    /// 键数量（去重后）
    pub unique_keys: usize,
    /// 平均每键的值数量
    pub avg_values_per_key: f64,
    /// 索引大小（字节）
    pub size_bytes: usize,
    /// 查询次数
    pub query_count: u64,
    /// 命中次数
    pub hit_count: u64,
}

impl IndexStats {
    /// 创建新的统计信息
    pub fn new() -> Self {
        Self::default()
    }

    /// 更新统计信息
    pub fn update(&mut self, entries: usize, unique_keys: usize, size_bytes: usize) {
        self.entries = entries;
        self.unique_keys = unique_keys;
        self.avg_values_per_key = if unique_keys > 0 {
            entries as f64 / unique_keys as f64
        } else {
            0.0
        };
        self.size_bytes = size_bytes;
    }

    /// 记录查询
    pub fn record_query(&mut self, hit: bool) {
        self.query_count += 1;
        if hit {
            self.hit_count += 1;
        }
    }

    /// 获取命中率
    pub fn hit_rate(&self) -> f64 {
        if self.query_count > 0 {
            self.hit_count as f64 / self.query_count as f64
        } else {
            0.0
        }
    }

    /// 重置统计信息
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl IndexMetadata {
    /// 创建新的索引元数据
    ///
    /// # 参数
    ///
    /// * `name` - 索引名称
    /// * `index_type` - 索引类型
    /// * `key_type_id` - 键类型标识符
    pub fn new(name: String, index_type: IndexType, key_type_id: TypeId) -> Self {
        Self {
            name,
            index_type,
            key_type_id,
            stats: IndexStats::new(),
            initialized: false,
        }
    }

    /// 标记索引为已初始化
    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }

    /// 检查键类型是否匹配
    ///
    /// # 参数
    ///
    /// * `expected_type_id` - 期望的键类型标识符
    pub fn matches_key_type(&self, expected_type_id: TypeId) -> bool {
        self.key_type_id == expected_type_id
    }

    /// 获取索引描述
    pub fn description(&self) -> String {
        format!(
            "{}({}) - {} entries, {:.1}% hit rate",
            self.name,
            self.index_type,
            self.stats.entries,
            self.stats.hit_rate() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_value_conversions() {
        let str_val: IndexValue = "hello".into();
        let u32_val: IndexValue = 42u32.into();
        let bool_val: IndexValue = true.into();

        match str_val {
            IndexValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string value"),
        }

        match u32_val {
            IndexValue::U32(n) => assert_eq!(n, 42),
            _ => panic!("Expected u32 value"),
        }

        match bool_val {
            IndexValue::Bool(b) => assert_eq!(b, true),
            _ => panic!("Expected bool value"),
        }
    }

    #[test]
    fn test_generic_index_definition() {
        type Person = (String, u32, String); // (name, age, profession)

        let age_index = GenericIndexDefinition::new(
            "age".to_string(),
            IndexType::Range,
            |person: &Person| person.1,
        );

        assert_eq!(age_index.name(), "age");
        assert_eq!(age_index.index_type(), IndexType::Range);

        let person = ("Alice".to_string(), 30, "Engineer".to_string());
        let age = age_index.extract(&person);
        assert_eq!(age, 30);
    }

    #[test]
    fn test_index_metadata() {
        let mut metadata = IndexMetadata::new(
            "test_index".to_string(),
            IndexType::Hash,
            TypeId::of::<String>(),
        );

        assert_eq!(metadata.name, "test_index");
        assert_eq!(metadata.index_type, IndexType::Hash);
        assert!(!metadata.initialized);

        metadata.mark_initialized();
        assert!(metadata.initialized);

        assert!(metadata.matches_key_type(TypeId::of::<String>()));
        assert!(!metadata.matches_key_type(TypeId::of::<u32>()));
    }

    #[test]
    fn test_index_stats() {
        let mut stats = IndexStats::new();

        // 初始状态
        assert_eq!(stats.query_count, 0);
        assert_eq!(stats.hit_count, 0);
        assert_eq!(stats.hit_rate(), 0.0);

        // 更新统计信息
        stats.update(100, 50, 1024);
        assert_eq!(stats.entries, 100);
        assert_eq!(stats.unique_keys, 50);
        assert_eq!(stats.avg_values_per_key, 2.0);
        assert_eq!(stats.size_bytes, 1024);

        // 记录查询
        stats.record_query(true);
        stats.record_query(false);
        stats.record_query(true);

        assert_eq!(stats.query_count, 3);
        assert_eq!(stats.hit_count, 2);
        assert!((stats.hit_rate() - 0.6666666666666666).abs() < 1e-10);

        // 重置
        stats.reset();
        assert_eq!(stats.query_count, 0);
        assert_eq!(stats.entries, 0);
    }
}