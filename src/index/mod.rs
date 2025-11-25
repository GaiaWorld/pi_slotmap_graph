/**
 * # 索引系统 (Index System)
 *
 * 本模块为 SlotMapGraph 提供高性能的索引功能，支持多种查询优化策略。
 *
 * ## 设计理念
 *
 * ### 为什么需要索引？
 *
 * **基础查询的问题**：
 * - 线性扫描所有顶点或边：O(n) 时间复杂度
 * - 复杂查询需要多次迭代：性能随数据规模线性下降
 * - 无统计信息：无法选择最优查询策略
 *
 * **索引的优势**：
 * - **O(log n) 查找**：哈希和树结构提供对数时间复杂度
 * - **范围查询**：支持高效的数值和时间范围查询
 * - **复合查询**：多条件组合查询的智能优化
 * - **内存优化**：比线性扫描更少的内存访问
 *
 * ## 索引类型
 *
 * ### 哈希索引 (HashIndex)
 * - **适用场景**：精确匹配查询
 * - **数据结构**：`HashMap<K, HashSet<VertexId/EdgeId>>`
 * - **性能**：O(1) 插入和查找
 * - **内存**：中等开销，支持一对多关系
 *
 * ### 范围索引 (RangeIndex)
 * - **适用场景**：数值范围、时间范围查询
 * - **数据结构**：`BTreeMap<K, HashSet<VertexId/EdgeId>>`
 * - **性能**：O(log n) 插入，O(log n + m) 范围查询
 * - **内存**：较高开销，但提供强大的查询能力
 *
 * ## 架构设计
 *
 * ```text,ignore
 * ┌─────────────────────────────────────────────────┐
 * │              Index Manager                       │
 * │         统一的索引管理接口                        │
 * ├─────────────────────────────────────────────────┤
 * │        Index Metadata Registry                   │
 * │       索引元数据注册和查询                        │
 * ├─────────────────────────────────────────────────┤
 * │    HashIndex    │    RangeIndex   │  Future...   │
 * │   (精确匹配)     │   (范围查询)     │  (扩展)      │
 * └─────────────────────────────────────────────────┘
 * ```
 *
 * ## 核心特性
 *
 * ### 类型安全
 * - **泛型设计**：支持任意可哈希和可比较的类型
 * - **编译时检查**：防止类型混淆和运行时错误
 * - **零成本抽象**：索引操作无额外运行时开销
 *
 * ### 内存效率
 * - **紧凑存储**：使用高效的集合数据结构
 * **自动清理**：删除元素时自动维护索引一致性
 * **延迟初始化**：按需创建索引，节省内存
 *
 * ### 查询优化
 * - **智能选择**：根据查询类型自动选择最优索引
 * - **组合查询**：支持多索引的联合查询
 * - **短路求值**：避免不必要的计算
 *
 * ## 使用示例
 *
 * ### 基本索引操作
 *
 * ```rust
 * use slotmap_graph::{SlotMapGraph, index::{IndexManager, IndexType}};
 *
 * let mut graph = SlotMapGraph::new();
 * let index_manager = graph.index_manager();
 *
 * // 创建顶点
 * let alice = graph.add_vertex(("Alice", 25, "Engineer"));
 * let bob = graph.add_vertex(("Bob", 30, "Designer"));
 * let charlie = graph.add_vertex(("Charlie", 35, "Manager"));
 *
 * // 创建哈希索引 - 按职业查询
 * index_manager.create_vertex_index(
 *     "profession",
 *     |vertex: &(String, u32, String)| vertex.2.clone(),
 *     IndexType::Hash
 * );
 *
 * // 创建范围索引 - 按年龄查询
 * index_manager.create_vertex_index(
 *     "age",
 *     |vertex: &(String, u32, String)| vertex.1,
 *     IndexType::Range
 * );
 *
 * // 使用索引查询
 * let engineers: Vec<_> = index_manager
 *     .query_vertex_hash("profession", "Engineer")
 *     .collect();
 *
 * let senior_people: Vec<_> = index_manager
 *     .query_vertex_range("age", 30..=40)
 *     .collect();
 * ```
 *
 * ## 性能特征
 *
 * ### 时间复杂度
 * - **索引创建**：O(n) - 遍历现有数据构建索引
 * - **哈希查询**：O(1) - 常数时间精确匹配
 * - **范围查询**：O(log n + k) - k为结果数量
 * - **索引更新**：O(1) 或 O(log n) - 取决于索引类型
 *
 * ### 空间复杂度
 * - **哈希索引**：O(n) - n为索引项数量
 * - **范围索引**：O(n) - 额外的树结构开销
 * - **元数据**：O(m) - m为索引定义数量
 *
 * ## 与 graph-api-simplegraph 的对比
 *
 * | 特性 | SlotMapGraph Index | SimpleGraph Index |
 * |------|-------------------|-------------------|
 * | 存储基础 | SlotMap + 索引 | Vector + 索引 |
 * | 索引更新 | 实时更新 | 批量更新 |
 * | 内存布局 | 更紧凑 | 稍高开销 |
 * | 动态性 | 高 | 中等 |
 * | 复杂度 | 简化版 | 功能完整 |
 */

pub mod hash;
pub mod range;
// pub mod manager;
pub mod metadata;
pub mod simple_query;
// pub mod smart_query_test;
// pub mod test_basic;


// 重新导出主要类型
pub use hash::HashIndex;
pub use range::RangeIndex;
// pub use manager::{IndexManager, QueryResult};
pub use metadata::{IndexMetadata, IndexDefinition, IndexValue};
pub use simple_query::SimpleVertexQuery;