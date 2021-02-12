use criterion::*;
mod parse_body;
mod parse_query;
mod regex;

criterion_group!(
    benches,
    regex::criterion_benchmark,
    parse_body::criterion_benchmark,
    parse_query::criterion_benchmark
);
criterion_main!(benches);
