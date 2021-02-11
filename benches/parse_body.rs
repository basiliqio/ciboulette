use criterion::Criterion;
use serde::Deserialize;
use std::time::Duration;

const VAL: &str = r#"
	{
		"data":
		[
			{
				"id": "6720877a-e27e-4e9e-9ac0-3fff4deb55f2",
				"type": "english",
				"attributes":
				{
					"hello": "world",
					"world": "the earth"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/relationships/planet",
						"related": "/planet/6720877a-e27e-4e9e-9ac0-3fff4deb55f2/english"
					  },
					  "data":
					  [
						  {
							"type": "planet",
							"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
						  }
					  ]
					}
				}
			},
			{
				"id": "568109e1-74e9-41b3-a10f-f103aba5e78c",
				"type": "english",
				"attributes":
				{
					"hello": "world2",
					"world": "the earth v2"
				},
				"relationships":
				{
					"planet":
					{
					  "links":
					  {
						"self": "/english/568109e1-74e9-41b3-a10f-f103aba5e78c/relationships/planet",
						"related": "/planet/568109e1-74e9-41b3-a10f-f103aba5e78c/english"
					  },
					  "data":
					  [
						  {
							"type": "planet",
							"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
						  }
					  ]
					}
				}
			}
		],
		"included":
		[
			{
				"id": "b922a277-aadb-4c4e-b13d-9c4c98b3ad80",
				"type": "planet",
				"attributes":
				{
					"p": "earth"
				},
				"links":
				{
					"self": "/planet/b922a277-aadb-4c4e-b13d-9c4c98b3ad80"
				}
			}
		]
	}
	"#;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json:api body");
    let bag = ciboulette_test_helper::gen_bag();

    group.sample_size(1000);
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(10));
    group.bench_with_input(
        criterion::BenchmarkId::new("body_complexe", "normal"),
        &VAL,
        |b, i| {
            b.iter(|| {
                let mut deserializer = serde_json::Deserializer::from_str(i);
                let doc = ciboulette::CibouletteTopLevelBuilder::deserialize(&mut deserializer)
                    .expect("no error");
                doc.build(&bag).expect("no error");
            })
        },
    );
    group.finish();
}