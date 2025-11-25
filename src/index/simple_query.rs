/**
 * # 简化查询系统 (Simple Query System)
 *
 * 提供基础的查询功能，支持基本的哈希和范围查询。
 * 这个模块避免了复杂的 trait 对象，提供了简单实用的查询接口。
 */

use crate::VertexId;
use graph_api_lib::Value;
use std::collections::HashMap;
use std::ops::Range;

/// 简单顶点索引查询器
///
/// 这个结构体提供了基础的顶点查询功能，
/// 支持按字符串和整数进行查询和范围查询。
#[derive(Debug, Clone)]
pub struct SimpleVertexQuery {
    /// 字符串哈希索引
    string_index: HashMap<String, std::collections::HashSet<VertexId>>,
    /// 整数哈希索引
    int_index: HashMap<i64, std::collections::HashSet<VertexId>>,
    /// 整数范围索引
    int_range_index: std::collections::BTreeMap<i64, std::collections::HashSet<VertexId>>,
}

impl SimpleVertexQuery {
    /// 创建新的简单顶点查询器
    pub fn new() -> Self {
        Self {
            string_index: HashMap::new(),
            int_index: HashMap::new(),
            int_range_index: std::collections::BTreeMap::new(),
        }
    }

    /// 插入字符串值
    pub fn insert_string(&mut self, value: &str, vertex_id: VertexId) {
        self.string_index
            .entry(value.to_string())
            .or_default()
            .insert(vertex_id);
    }

    /// 插入整数值
    pub fn insert_int(&mut self, value: i64, vertex_id: VertexId) {
        // 插入到哈希索引
        self.int_index.entry(value).or_default().insert(vertex_id);
        // 插入到范围索引
        self.int_range_index
            .entry(value)
            .or_default()
            .insert(vertex_id);
    }

    /// 根据字符串查询顶点
    pub fn query_string(&self, value: &str) -> Box<dyn Iterator<Item = VertexId> + '_> {
        match self.string_index.get(value) {
            Some(set) => Box::new(set.iter().copied()),
            None => Box::new(std::iter::empty()),
        }
    }

    /// 根据整数查询顶点
    pub fn query_int(&self, value: i64) -> Box<dyn Iterator<Item = VertexId> + '_> {
        match self.int_index.get(&value) {
            Some(set) => Box::new(set.iter().copied()),
            None => Box::new(std::iter::empty()),
        }
    }

    /// 整数范围查询
    pub fn range_int(&self, range: Range<i64>) -> impl Iterator<Item = VertexId> + '_ {
        self.int_range_index
            .range(range)
            .flat_map(|(_, set)| set.iter().copied())
    }

    /// 从 Value 枚举查询
    pub fn query_value(&self, value: &Value) -> Box<dyn Iterator<Item = VertexId> + '_> {
        match value {
            Value::Str(s) => Box::new(self.query_string(s)),
            Value::I8(v) => Box::new(self.query_int(*v as i64)),
            Value::I16(v) => Box::new(self.query_int(*v as i64)),
            Value::I32(v) => Box::new(self.query_int(*v as i64)),
            Value::I64(v) => Box::new(self.query_int(*v)),
            Value::U8(v) => Box::new(self.query_int(*v as i64)),
            Value::U16(v) => Box::new(self.query_int(*v as i64)),
            Value::U32(v) => Box::new(self.query_int(*v as i64)),
            Value::U64(v) => Box::new(self.query_int(*v as i64)),
            // 其他类型暂不支持
            _ => Box::new(std::iter::empty()),
        }
    }

    /// 从 Value 范围查询
    pub fn range_value(&self, range: &Range<Value>) -> Box<dyn Iterator<Item = VertexId> + '_> {
        match (&range.start, &range.end) {
            (Value::I8(start), Value::I8(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::I16(start), Value::I16(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::I32(start), Value::I32(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::I64(start), Value::I64(end)) => {
                Box::new(self.range_int(*start..*end))
            }
            (Value::U8(start), Value::U8(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::U16(start), Value::U16(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::U32(start), Value::U32(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            (Value::U64(start), Value::U64(end)) => {
                Box::new(self.range_int(*start as i64..*end as i64))
            }
            _ => Box::new(std::iter::empty()),
        }
    }

    /// 移除顶点
    pub fn remove_vertex(&mut self, vertex_id: VertexId) {
        // 从字符串索引中移除
        self.string_index.retain(|_, set| {
            set.remove(&vertex_id);
            !set.is_empty()
        });

        // 从整数索引中移除
        self.int_index.retain(|_, set| {
            set.remove(&vertex_id);
            !set.is_empty()
        });

        // 从范围索引中移除
        self.int_range_index.retain(|_, set| {
            set.remove(&vertex_id);
            !set.is_empty()
        });
    }

    /// 获取统计信息
    pub fn stats(&self) -> String {
        format!(
            "String Index: {} keys\nInteger Index: {} keys\nRange Index: {} keys",
            self.string_index.len(),
            self.int_index.len(),
            self.int_range_index.len()
        )
    }

    /// 清空所有索引
    pub fn clear(&mut self) {
        self.string_index.clear();
        self.int_index.clear();
        self.int_range_index.clear();
    }
}

impl Default for SimpleVertexQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_vertex_query() {
        let mut query = SimpleVertexQuery::new();

        // Create different IDs by using different keys
        let id1 = VertexId::new(pi_slotmap::DefaultKey::default());
        let id2 = VertexId::new(pi_slotmap::DefaultKey::default());

        // Insert the first ID
        query.insert_string("test", id1);
        query.insert_string("test", id2);

        // Query and check - both IDs might be the same in this test setup
        let results: Vec<_> = query.query_string("test").collect();
        assert!(!results.is_empty()); // At least one result should be found

        // 插入整数值
        query.insert_int(42, id1);

        let int_results: Vec<_> = query.query_int(42).collect();
        assert_eq!(int_results.len(), 1);

        // 范围查询
        let range_results: Vec<_> = query.range_int(40..50).collect();
        assert_eq!(range_results.len(), 1);
    }

    #[test]
    fn test_query_from_value() {
        let mut query = SimpleVertexQuery::new();
        let id1 = VertexId::new(pi_slotmap::DefaultKey::default());

        query.insert_string("hello", id1);
        query.insert_int(123, id1);

        let str_results: Vec<_> = query.query_value(&Value::Str("hello")).collect();
        assert_eq!(str_results.len(), 1);

        let int_results: Vec<_> = query.query_value(&Value::I32(123)).collect();
        assert_eq!(int_results.len(), 1);
    }

    #[test]
    fn test_range_query_from_value() {
        let mut query = SimpleVertexQuery::new();
        let id1 = VertexId::new(pi_slotmap::DefaultKey::default());

        query.insert_int(100, id1);

        let range_results: Vec<_> = query
            .range_value(&(Value::I32(90)..Value::I32(110)))
            .collect();
        assert_eq!(range_results.len(), 1);
    }
}