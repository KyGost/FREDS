use {
    criterion::*,
    freds::{Reader, Write},
    serde_json::{from_str, Value},
    std::fs::File,
    tokio::runtime::Runtime,
};

fn bench(criterion: &mut Criterion) {
    println!("Parsing JSON");
    let json: Value = from_str(include_str!(
        "../src/implementations/serde_json/test_jsonorg.json"
    ))
    .unwrap();

    let mut group = criterion.benchmark_group("write");
    group.bench_function("write", |benchmarker| {
        use std::io::prelude::*;
        benchmarker.iter(|| {
            let writer = json.clone().write();
            let mut buffer = File::create("test_jsonorg.freds").unwrap();
            buffer.write(&writer).unwrap();
        });
    });
    group.finish();

    let mut group = criterion.benchmark_group("read");
    group.bench_function("read", |benchmarker| {
        benchmarker.iter(|| {
            let runtime = Runtime::new().unwrap();
            let mut reader = runtime
                .block_on(Reader::from_file("test_jsonorg.freds"))
                .unwrap();
            let result: Value = runtime.block_on(reader.get(reader.core)).unwrap();
            assert_eq!(json, result);
        });
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bench
}
criterion_main!(benches);
