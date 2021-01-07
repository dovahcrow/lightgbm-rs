use failure::Error;
use fehler::throws;
use lightgbm::{DType, LightGBM, PredictType};

#[throws(Error)]
fn main() {
    let model = LightGBM::from_file("examples/iris.model").unwrap();
    println!("# classes {}, # iterations {}", model.num_classes()?, model.num_iterations());

    let feat = vec![7.7, 2.8, 6.7, 2.];

    let pred = model.predict_for_mat_single_row(feat.as_slice(), PredictType::Normal, 0, -1, None)?;
    println!("{:?}", pred);

    // or the fast version
    let fchandle = model.predict_for_mat_single_row_fast_init(PredictType::Normal, 0, -1, DType::Float64, feat.len() as i32, None)?;

    let pred = model.predict_for_mat_single_row_fast(fchandle, feat.as_slice())?;
    println!("{:?}", pred);
}
