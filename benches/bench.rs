use {
    criterion::*,
    freds::{Reader, Write},
    serde_json::{from_str, Value},
    std::fs::File,
    tokio::runtime::Runtime,
};

fn bench(criterion: &mut Criterion) {
    println!("Parsing JSON");
    let jsonorg: Value = from_str(include_str!(
        "../src/implementations/serde_json/test_jsonorg.json"
    ))
    .unwrap();
    let book: Value = from_str(include_str!(
        "../src/implementations/serde_json/test_book.json"
    ))
    .unwrap();
    let misc: Value = from_str(include_str!(
        "../src/implementations/serde_json/test_misc.json"
    ))
    .unwrap();

    let runtime = Runtime::new().unwrap();

    let mut group = criterion.benchmark_group("write");
    group.bench_function("jsonorg", |benchmarker| {
        use std::io::prelude::*;
        benchmarker.iter(|| {
            let writer = jsonorg.clone().write();
            let mut buffer = File::create("test_jsonorg.freds").unwrap();
            buffer.write(&writer).unwrap();
        });
    });
    group.bench_function("book", |benchmarker| {
        //use std::io::prelude::*;
        benchmarker.iter(|| {
            let _writer = book.clone().write();
            //let mut buffer = File::create("test_book.freds").unwrap();
            //buffer.write(&writer).unwrap();
        });
    });
    group.bench_function("misc", |benchmarker| {
        //use std::io::prelude::*;
        benchmarker.iter(|| {
            let _writer = misc.clone().write();
            //let mut buffer = File::create("test_misc.freds").unwrap();
            //buffer.write(&writer).unwrap();
        });
    });
    group.finish();

    let mut group = criterion.benchmark_group("read");
    group.bench_function("book", |benchmarker| {
        benchmarker.iter(|| {
            let _reader: Reader<Value> = runtime
                .block_on(Reader::from_file("test_book.freds"))
                .unwrap();
        });
    });
    group.bench_function("misc", |benchmarker| {
        benchmarker.iter(|| {
            let _reader: Reader<Value> = runtime
                .block_on(Reader::from_file("test_misc.freds"))
                .unwrap();
        });
    });
    group.bench_function("jsonorg", |benchmarker| {
        benchmarker.iter(|| {
            let _reader: Reader<Value> = runtime
                .block_on(Reader::from_file("test_jsonorg.freds"))
                .unwrap();
        });
    });
    group.finish();

    let mut group = criterion.benchmark_group("get");
    let mut reader = runtime
        .block_on(Reader::from_file("test_book.freds"))
        .unwrap();
    group.bench_function("book", |benchmarker| {
        benchmarker.iter(|| {
            let _result: Value = runtime.block_on(reader.get(reader.core)).unwrap();
        });
    });
    let mut reader = runtime
        .block_on(Reader::from_file("test_misc.freds"))
        .unwrap();
    group.bench_function("misc", |benchmarker| {
        benchmarker.iter(|| {
            let _result: Value = runtime.block_on(reader.get(reader.core)).unwrap();
        });
    });
    let mut reader = runtime
        .block_on(Reader::from_file("test_jsonorg.freds"))
        .unwrap();
    group.bench_function("jsonorg", |benchmarker| {
        benchmarker.iter(|| {
            let _result: Value = runtime.block_on(reader.get(reader.core)).unwrap();
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
