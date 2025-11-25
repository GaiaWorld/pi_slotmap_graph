/**
 * # 基本操作基准测试
 * 对应 basic_operations.rs 中的测试
 */

use graph_api_lib::{Graph, VertexSearch, VertexReference};
use graph_api_simplegraph::SimpleGraph;
use pi_slotmap_graph::SlotMapGraph;
use std::time::Instant;

type TestData = u32;

#[test]
fn test_basic_addition_benchmark() {
    println!("📊 添加性能基准测试");
    println!("====================");

    let sizes = [1000, 5000, 10000];

    for &size in &sizes {
        println!("\n数据规模: {} 顶点", size);

        // SlotMapGraph 测试
        let start = Instant::now();
        let mut graph: SlotMapGraph<TestData, TestData> = SlotMapGraph::new();

        for i in 0..size {
            graph.add_vertex(i as TestData);
        }

        let slotmap_time = start.elapsed();
        let slotmap_throughput = size as f64 / slotmap_time.as_secs_f64();

        // SimpleGraph 测试
        let start = Instant::now();
        let mut graph: SimpleGraph<TestData, TestData> = SimpleGraph::new();

        for i in 0..size {
            graph.add_vertex(i as TestData);
        }

        let simple_time = start.elapsed();
        let simple_throughput = size as f64 / simple_time.as_secs_f64();

        println!("  SlotMapGraph: {:?} ({:.0} vertices/sec)", slotmap_time, slotmap_throughput);
        println!("  SimpleGraph: {:?} ({:.0} vertices/sec)", simple_time, simple_throughput);

        if slotmap_time < simple_time {
            let speedup = simple_time.as_secs_f64() / slotmap_time.as_secs_f64();
            println!("  ⚡ SlotMapGraph 快 {:.2}x", speedup);
        } else {
            let speedup = slotmap_time.as_secs_f64() / simple_time.as_secs_f64();
            println!("  ⚡ SimpleGraph 快 {:.2}x", speedup);
        }
    }
}

#[test]
fn test_basic_query_benchmark() {
    println!("\n🔍 查询性能基准测试");
    println!("====================");

    let size = 10000;
    println!("数据规模: {} 顶点", size);

    // 创建测试数据
    let mut slotmap_graph: SlotMapGraph<TestData, TestData> = SlotMapGraph::new();
    let mut simple_graph: SimpleGraph<TestData, TestData> = SimpleGraph::new();

    for i in 0..size {
        slotmap_graph.add_vertex(i as TestData);
        simple_graph.add_vertex(i as TestData);
    }

    // SlotMapGraph 查询测试（查找偶数）
    let start = Instant::now();
    let mut count = 0;
    let search = VertexSearch::scan();

    for vertex_ref in slotmap_graph.vertices(&search) {
        if *vertex_ref.weight() % 2 == 0 {
            count += 1;
        }
    }

    let slotmap_time = start.elapsed();

    // SimpleGraph 查询测试
    let start = Instant::now();
    let mut count = 0;
    let search = VertexSearch::scan();

    for vertex_ref in simple_graph.vertices(&search) {
        if *vertex_ref.weight() % 2 == 0 {
            count += 1;
        }
    }

    let simple_time = start.elapsed();

    println!("  SlotMapGraph: {:?} (找到 {} 个偶数)", slotmap_time, count);
    println!("  SimpleGraph: {:?} (找到 {} 个偶数)", simple_time, count);

    if slotmap_time < simple_time {
        let speedup = simple_time.as_secs_f64() / slotmap_time.as_secs_f64();
        println!("  ⚡ SlotMapGraph 查询快 {:.2}x", speedup);
    } else {
        let speedup = slotmap_time.as_secs_f64() / simple_time.as_secs_f64();
        println!("  ⚡ SimpleGraph 查询快 {:.2}x", speedup);
    }
}

#[test]
fn test_basic_iteration_benchmark() {
    println!("\n🔄 遍历性能基准测试");
    println!("====================");

    let size = 10000;
    println!("数据规模: {} 顶点", size);

    // 创建测试数据
    let mut slotmap_graph: SlotMapGraph<TestData, TestData> = SlotMapGraph::new();
    let mut simple_graph: SimpleGraph<TestData, TestData> = SimpleGraph::new();

    for i in 0..size {
        slotmap_graph.add_vertex(i as TestData);
        simple_graph.add_vertex(i as TestData);
    }

    // SlotMapGraph 遍历测试
    let start = Instant::now();
    let mut sum = 0u64;
    let search = VertexSearch::scan();

    for vertex_ref in slotmap_graph.vertices(&search) {
        sum += *vertex_ref.weight() as u64;
    }

    let slotmap_time = start.elapsed();

    // SimpleGraph 遍历测试
    let start = Instant::now();
    let mut sum = 0u64;
    let search = VertexSearch::scan();

    for vertex_ref in simple_graph.vertices(&search) {
        sum += *vertex_ref.weight() as u64;
    }

    let simple_time = start.elapsed();

    let throughput_slotmap = size as f64 / slotmap_time.as_secs_f64();
    let throughput_simple = size as f64 / simple_time.as_secs_f64();

    println!("  SlotMapGraph: {:?} (吞吐量: {:.0}/sec)", slotmap_time, throughput_slotmap);
    println!("  SimpleGraph: {:?} (吞吐量: {:.0}/sec)", simple_time, throughput_simple);

    if slotmap_time < simple_time {
        let speedup = simple_time.as_secs_f64() / slotmap_time.as_secs_f64();
        println!("  ⚡ SlotMapGraph 遍历快 {:.2}x", speedup);
    } else {
        let speedup = slotmap_time.as_secs_f64() / simple_time.as_secs_f64();
        println!("  ⚡ SimpleGraph 遍历快 {:.2}x", speedup);
    }
}

#[test]
fn test_basic_memory_benchmark() {
    println!("\n💾 内存效率基准测试");
    println!("====================");

    let size = 50000;
    println!("数据规模: {} 顶点", size);

    // SlotMapGraph 大数据集测试
    let start = Instant::now();
    let mut graph: SlotMapGraph<TestData, TestData> = SlotMapGraph::new();

    for i in 0..size {
        graph.add_vertex(i as TestData);
    }

    let slotmap_time = start.elapsed();

    // SimpleGraph 大数据集测试
    let start = Instant::now();
    let mut graph: SimpleGraph<TestData, TestData> = SimpleGraph::new();

    for i in 0..size {
        graph.add_vertex(i as TestData);
    }

    let simple_time = start.elapsed();

    let search = VertexSearch::scan();
    let count: usize = graph.vertices(&search).count();

    println!("  SlotMapGraph: {:?} (创建 {} 顶点)", slotmap_time, size);
    println!("  SimpleGraph: {:?} (创建 {} 顶点)", simple_time, count);

    let throughput_slotmap = size as f64 / slotmap_time.as_secs_f64();
    let throughput_simple = size as f64 / simple_time.as_secs_f64();

    println!("  大数据集吞吐量:");
    println!("    SlotMapGraph: {:.0} vertices/sec", throughput_slotmap);
    println!("    SimpleGraph: {:.0} vertices/sec", throughput_simple);

    if slotmap_time < simple_time {
        let speedup = simple_time.as_secs_f64() / slotmap_time.as_secs_f64();
        println!("  ⚡ SlotMapGraph 大数据集快 {:.2}x", speedup);
    } else {
        let speedup = slotmap_time.as_secs_f64() / simple_time.as_secs_f64();
        println!("  ⚡ SimpleGraph 大数据集快 {:.2}x", speedup);
    }
}