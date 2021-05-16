use criterion::*;
mod parse_body;
mod parse_query;
criterion_group!(
    benches,
    parse_body::criterion_benchmark,
    parse_query::criterion_benchmark
);
criterion_main!(benches);
