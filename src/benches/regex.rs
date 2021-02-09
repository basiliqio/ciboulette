use super::*;
use criterion::Criterion;
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simple object");

    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    group.bench_with_input(
        criterion::BenchmarkId::new("regex_member_name", "normal"),
        &"hello_world",
        |b, i| b.iter(|| ciboulette::check_member_name(i)),
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("regex_member_name", "unicode"),
        &"hello_😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀_world",
        |b, i| b.iter(|| ciboulette::check_member_name(i)),
    );
    group.finish();
}
