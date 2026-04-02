use aethelgard_avl::SovereignMap;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("SovereignMap Operations");
    let map = SovereignMap::new();

    for i in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("insert", i), i, |b, &i| {
            b.iter(|| {
                let key = format!("key_{}", i);
                let val = format!("val_{}", i);
                let _ = map.insert(key, val);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("get", i), i, |b, &i| {
            let key = format!("key_{}", i);
            b.iter(|| {
                let _ = map.get(&key);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
