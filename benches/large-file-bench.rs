use criterion::{black_box, criterion_group, criterion_main, Criterion};
use twojson::{Config,run};

fn run_benchmark(c: &mut Criterion) {   
    c.bench_function(
        "Converting large-file.csv",
     |b| b.iter(|| run(black_box(Config {
        file_path: String::from("./sample-data/large-file.csv"),
    })))
    );
}

criterion_group!(benches, run_benchmark);
criterion_main!(benches);