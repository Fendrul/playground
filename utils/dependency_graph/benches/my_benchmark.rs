use std::time::Duration;
use dependency_graph::DepGraph;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// Import your DepGraph code here

fn bench_add_node(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_node");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("add 1000 nodes", |b| {
        b.iter(|| {
            let mut graph = DepGraph::new();
            for i in 0..1000 {
                graph.get_or_add_node(black_box(i));
            }
        });
    });

    group.finish();
}

fn bench_add_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_edge");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("add 1000 edges", |b| {
        b.iter(|| {
            let mut graph = DepGraph::new();
            let nodes: Vec<_> = (0..1000)
                .map(|i| graph.get_or_add_node(i))
                .filter_map(Result::ok)
                .collect();
            for i in 0..999 {
                DepGraph::add_edge(&nodes[i], &nodes[i+1]).unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_add_node, bench_add_edge);
criterion_main!(benches);
