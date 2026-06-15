use backend::{Backend, CpuBackend};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_matmul_f32(c: &mut Criterion) {
    let m = 512_usize;
    let k = 512;
    let n = 512;

    let a: Vec<f32> = (0..m * k).map(|i| (i % 17) as f32 * 0.01).collect();
    let b: Vec<f32> = (0..k * n).map(|i| (i % 13) as f32 * 0.02).collect();

    let backend = CpuBackend;
    
    let flops = (2 * m * k * n) as u64; // 2 flops per multiplication (mul and add)
    
    let mut group = c.benchmark_group("matmul_f32");
    group.throughput(Throughput::Elements(flops));
    group.bench_function("cpu_512", |bencher| {
        bencher.iter(|| {
            backend.matmul_f32(
                black_box(&a),
                black_box(&b),
                black_box(m),
                black_box(k),
                black_box(n),
            )
        });
    });
    group.finish();
}

criterion_group!(benches, bench_matmul_f32);
criterion_main!(benches);