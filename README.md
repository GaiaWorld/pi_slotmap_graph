# SlotMap Graph

基于 `pi_slotmap` 的高性能图数据库实现，完全兼容 `graph-api-lib` 标准。

## 🚀 特性

- **高性能存储**: 基于 `pi_slotmap::SlotMap` 的 O(1) 插入、删除和查找
- **类型安全**: 强类型的泛型设计，支持任意顶点和边类型
- **内存高效**: 紧凑的内存布局和自动重用机制
- **标准兼容**: 完全符合 `graph-api-lib` 规范
- **线程安全**: 支持 `Send` 和 `Sync` trait
- **可扩展**: 模块化设计，易于扩展和定制

## 📁 模块结构

```
slotmap_graph/
├── mod.rs              # 模块导出和文档
├── README.md           # 本文档
├── id/                 # 标识符系统 (ID Layer)
│   ├── mod.rs         # ID模块导出和类型转换
│   ├── vertex_id.rs   # VertexId实现
│   ├── edge_id.rs     # EdgeId实现
│   └── edge_info.rs   # EdgeInfo连接信息
├── storage/            # 存储层 (Storage Layer)
│   ├── mod.rs         # 存储接口和统计
│   ├── vertex.rs      # VertexContainer顶点存储
│   ├── edge.rs        # EdgeContainer边存储
│   └── container.rs   # Container通用容器
├── reference/          # 引用层 (Reference Layer)
│   └── mod.rs         # 引用类型重新导出
├── iteration/          # 迭代层 (Iteration Layer)
│   └── mod.rs         # 迭代器接口
├── graph.rs            # 图核心实现 (Graph Layer)
└── tests.rs            # 集成测试
```

## 🏗️ 架构设计

### 分层架构

```
┌─────────────────────────────────────────┐
│           Graph Layer (graph.rs)          │  ← 图操作接口
├─────────────────────────────────────────┤
│         Reference Layer (reference/)      │  ← 安全引用系统
├─────────────────────────────────────────┤
│        Iteration Layer (iteration/)       │  ← 高效遍历能力
├─────────────────────────────────────────┤
│         Storage Layer (storage/)          │  ← 高性能存储
├─────────────────────────────────────────┤
│            ID Layer (id/)                │  ← 类型安全标识
└─────────────────────────────────────────┘
```

### 核心组件

#### ID System (`id/`)
- **VertexId**: 基于 `pi_slotmap::DefaultKey` 的顶点标识符
- **EdgeId**: 基于 `pi_slotmap::DefaultKey` 的边标识符
- **EdgeInfo**: 存储边的起点和终点信息

#### Storage Layer (`storage/`)
- **VertexContainer<V<T>>**: 高性能顶点存储容器
- **EdgeContainer<E<T>>**: 边存储容器，包含连接信息管理
- **Container Interface**: 通用存储接口，支持索引和查询

#### Reference System (`reference/`)
- **VertexReference**: 不可变顶点引用，支持安全访问和投影
- **VertexReferenceMut**: 可变顶点引用，支持安全修改
- **EdgeReference**: 不可变边引用，包含连接信息
- **EdgeReferenceMut**: 可变边引用，支持安全修改

#### Iteration Layer (`iteration/`)
- **VertexIter**: 高效的顶点迭代器，支持过滤和限制
- **EdgeIter**: 边迭代器，支持方向过滤和标签过滤

#### Graph Core (`graph.rs`)
- **SlotMapGraph**: 主要的图实现，整合所有功能模块

## 🔧 核心功能

### 基本图操作

```rust
use slotmap_graph::SlotMapGraph;
use graph_api_lib::{Element, Graph, VertexSearch};

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Element for Person {
    type Label = ();
    fn label(&self) -> Self::Label { () }
}

let mut graph = SlotMapGraph::<Person, String>::new();

// 添加顶点
let alice = graph.add_vertex(Person {
    name: "Alice".to_string(),
    age: 30,
});

let bob = graph.add_vertex(Person {
    name: "Bob".to_string(),
    age: 25,
});

// 添加边
let friendship = graph.add_edge(alice, bob, "friends".to_string());

// 查询顶点
if let Some(alice_ref) = graph.vertex(alice) {
    println!("{} is {} years old", alice_ref.weight().name, alice_ref.weight().age);
}

// 遍历顶点
for person_ref in graph.vertices(&VertexSearch::scan()) {
    println!("Found: {}", person_ref.weight().name);
}
```

### 边查询

```rust
use graph_api_lib::{EdgeSearch, Direction};

// 查询出边
for edge_ref in graph.edges(alice, &EdgeSearch::scan().outgoing()) {
    println!("Alice knows someone");
}

// 按标签查询边
for edge_ref in graph.edges(alice, &EdgeSearch::label("friends")) {
    println!("Friendship relation");
}
```

### 图分析

```rust
// 度数计算
let out_degree = graph.out_degree(alice);
let in_degree = graph.in_degree(alice);
let degree = graph.degree(alice);

// 邻接查询
let outgoing = graph.outgoing_edges(alice);
let incoming = graph.incoming_edges(alice);
let adjacent = graph.adjacent_edges(alice);

// 边存在性检查
if graph.has_edge(alice, bob) {
    println!("Alice and Bob are connected");
}
```

## 📈 性能特性

### 时间复杂度
- **插入顶点**: O(1)
- **插入边**: O(1)
- **删除顶点**: O(1)
- **删除边**: O(1)
- **顶点查询**: O(1)
- **边查询**: O(1)
- **度数计算**: O(degree)

### 空间复杂度
- **顶点存储**: O(|V|)
- **边存储**: O(|E|)
- **连接信息**: O(|E|)

### 内存优化
- 使用 `pi_slotmap` 的紧凑存储
- 自动内存重用机制
- 最小化内存碎片

## 🧪 测试

运行测试套件：

```bash
# 运行所有测试
cargo test --package pi_graph

# 运行特定模块测试
cargo test slotmap_graph

# 运行性能测试
cargo test --package pi_graph --release
```

## 📊 与其他实现的比较

| 特性 | SlotMapGraph | SimpleGraph | CsrGraph |
|------|-------------|-------------|----------|
| 存储方式 | SlotMap | Vector + Index | CSR |
| 插入性能 | O(1) | O(1) | O(log n) |
| 删除性能 | O(1) | O(1) | O(log n) |
| 内存使用 | 紧凑 | 中等 | 高度优化 |
| 查询性能 | 优秀 | 良好 | 优秀 |
| 动态性 | 高 | 中等 | 低 |

## 🔮 扩展性

### 自定义顶点和边类型

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomVertex {
    id: u64,
    data: String,
}

impl Element for CustomVertex {
    type Label = CustomLabel;
    fn label(&self) -> Self::Label { CustomLabel::Vertex }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomEdge {
    weight: f64,
    metadata: HashMap<String, String>,
}

impl Element for CustomEdge {
    type Label = CustomLabel;
    fn label(&self) -> Self::Label { CustomLabel::Edge }
}
```

### 索引支持

```rust
// 为顶点添加自定义索引
impl SlotMapGraph<CustomVertex, CustomEdge> {
    pub fn index_by_name(&self, name: &str) -> Vec<VertexId> {
        self.vertices()
            .filter(|v| v.weight().data == name)
            .map(|v| v.id())
            .collect()
    }
}
```

## 📄 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📞 联系方式

如有问题或建议，请联系项目维护者。