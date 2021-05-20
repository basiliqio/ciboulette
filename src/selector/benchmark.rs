extern crate ciboulette;
use criterion::Criterion;
use criterion::*;
use serde::Deserialize;
use std::time::Duration;
criterion_group!(benches, selector,);
criterion_main!(benches);

const VAL_MULTI: &str = r#"
		{
			"hello": ["world", "toto"]
		}
	"#;

const VAL_SINGLE: &str = r#"
	{
		"hello": ["world", "toto"]
	}
"#;

#[derive(Debug, Clone, Deserialize)]
struct TotoSelector {
    hello: ciboulette::CibouletteSelector<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DummySelector<T> {
    Single(T),
    Multi(Vec<T>),
}

#[derive(Debug, Clone, Deserialize)]
struct TotoSerde {
    hello: DummySelector<String>,
}

pub fn selector(c: &mut Criterion) {
    let mut group = c.benchmark_group("ciboulette selector");

    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));
    group.bench_with_input(
        criterion::BenchmarkId::new("selector_serde", "single"),
        &VAL_SINGLE,
        |b, i| {
            b.iter(|| {
                let _res: TotoSerde = serde_json::from_str(i).unwrap();
            })
        },
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("selector_serde", "multi"),
        &VAL_MULTI,
        |b, i| {
            b.iter(|| {
                let _res: TotoSerde = serde_json::from_str(i).unwrap();
            })
        },
    );

    group.bench_with_input(
        criterion::BenchmarkId::new("selector_custom", "single"),
        &VAL_SINGLE,
        |b, i| {
            b.iter(|| {
                let _res: TotoSelector = serde_json::from_str(i).unwrap();
            })
        },
    );
    group.bench_with_input(
        criterion::BenchmarkId::new("selector_custom", "multi"),
        &VAL_MULTI,
        |b, i| {
            b.iter(|| {
                let _res: TotoSelector = serde_json::from_str(i).unwrap();
            })
        },
    );
    group.finish();
}
