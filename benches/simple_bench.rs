use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lightgbm::{LightGBM, PredictType};

fn simple_bench(c: &mut Criterion) {
    let model = LightGBM::from_file("examples/iris.model").unwrap();

    let feat = vec![7.7, 2.8, 6.7, 2.];
    c.bench_function("simple", |b| {
        b.iter(|| model.predict_for_mat_single_row(black_box(feat.as_slice()), PredictType::Normal, 0, -1, None).unwrap())
    });
}

criterion_group!(benches, simple_bench);
criterion_main!(benches);
