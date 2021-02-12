use criterion::Criterion;
use std::time::Duration;

const VAL: &str = r#"include=comments&fields%5Bcomments%5D=body&fields[peoples]=first-name,last-name&sort=first-name&page[other]=HAHA&page[cursor]=988dfc35-4096-4da9-a22e-41d5e4348ae6&filter=toto&filter[peoples]=tutu&meta=ok"#;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json:api query");
    let bag = ciboulette_test_helper::gen_bag();
    let rt = bag.map().get("peoples").unwrap();

    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(10));
    group.bench_with_input(
        criterion::BenchmarkId::new("query", "normal"),
        &VAL,
        |b, i| {
            b.iter(|| {
                let doc: ciboulette::CibouletteQueryParametersBuilder =
                    serde_urlencoded::from_str(i).expect("no error");
                doc.build(&bag, Some(rt)).expect("no error");
            })
        },
    );
    group.finish();
}
