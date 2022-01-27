use criterion::{criterion_group, criterion_main, Criterion};
use serde::Deserialize;
use std::fs::File;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct EstimateItem {
    assembly_quantity: u32,
    item: Uuid,
    item_quantity: u32,
    price: f64,
}

fn estimate_items() -> Vec<EstimateItem> {
    let file = File::open("benches/SampleEstimateLarge.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut estimate = Vec::new();

    for result in rdr.deserialize() {
        let record: EstimateItem = result.unwrap();

        estimate.push(record)
    }

    estimate
}

fn run() {
    // wrap with black_box()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Setup

    c.bench_function("bunch name", |b| b.iter(|| run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
