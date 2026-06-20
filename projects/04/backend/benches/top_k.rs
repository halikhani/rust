use backend::{top_k_heap, top_k_sort};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

const N: usize = 32_000;
const K: usize = 50; // for topk


fn make_logits() -> Vec<f32> {
    (0..N).map(|i| ((i * 13) %1000) as f32 * 0.001).collect()
}

fn bench_top_k(c: &mut Criterion) {
    let logits = make_logits();
    let mut group = c.benchmark_group("top_k");
    group.throughput(Throughput::Elements(N as u64)); // N elements per iteration

    group.bench_function("sort_n_log_n", |b| {
        b.iter(|| {
            top_k_sort(black_box(&logits), black_box(K));
        });
    });
    group.bench_function("heap_n_log_k", |b| {
        b.iter(|| {
            top_k_heap(black_box(&logits), black_box(K));
        });
    });
    group.finish();
}

criterion_group!(benches, bench_top_k);
criterion_main!(benches);