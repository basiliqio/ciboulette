use criterion::*;
mod regex;

criterion_group!(benches, regex::criterion_benchmark);
criterion_main!(benches);
