mod bindings;
mod errors;
mod fast_config;
mod sys;

use crate::bindings::{BoosterHandle, LGBM_BoosterFree};
pub use crate::fast_config::FastConfig;
use crate::sys::{
    booster_create_from_modelfile, booster_get_num_classes, booster_load_model_from_string, booster_predict_for_mat_single_row, booster_predict_for_mat_single_row_fast,
    booster_predict_for_mat_single_row_fast_init,
};
pub use crate::sys::{DType, PredictType};
use failure::Error;
use fehler::throws;

pub struct LightGBM {
    handle: BoosterHandle,
    ntrees: i32,
}

// According to https://github.com/microsoft/LightGBM/issues/2201, BoosterHandle is Sync + Send
unsafe impl Send for LightGBM {}
unsafe impl Sync for LightGBM {}

impl LightGBM {
    #[throws(Error)]
    pub fn from_file(filename: &str) -> LightGBM {
        let (handle, ntrees) = booster_create_from_modelfile(filename)?;

        LightGBM { handle, ntrees }
    }

    #[throws(Error)]
    pub fn from_string(model_def: &str) -> LightGBM {
        let (handle, ntrees) = booster_load_model_from_string(model_def)?;

        LightGBM { handle, ntrees }
    }

    pub fn num_iterations(&self) -> i32 {
        self.ntrees
    }

    #[throws(Error)]
    pub fn num_classes(&self) -> i32 {
        booster_get_num_classes(self.handle)?
    }

    #[throws(Error)]
    pub fn predict_for_mat_single_row<'a, T>(&self, data: &[f64], predict_type: PredictType, start_iteration: i32, num_iteration: i32, params: T) -> Vec<f64>
    where
        T: Into<Option<&'a str>>,
    {
        let params = params.into().unwrap_or("");

        let mut out = vec![0.; self.num_classes()? as usize * 1];
        booster_predict_for_mat_single_row(self.handle, data, predict_type, start_iteration, num_iteration, params, out.as_mut())?;
        out
    }

    #[throws(Error)]
    pub fn predict_for_mat_single_row_fast_init<'a, T>(
        &'a self,
        predict_type: PredictType,
        start_iteration: i32,
        num_iteration: i32,
        data_type: DType,
        ncol: i32,
        params: T,
    ) -> FastConfig<'a>
    where
        T: Into<Option<&'a str>>,
    {
        let params = params.into().unwrap_or("");
        let fchandle = booster_predict_for_mat_single_row_fast_init(self.handle, predict_type, start_iteration, num_iteration, data_type, ncol, params)?;
        FastConfig::new(fchandle)
    }

    #[throws(Error)]
    pub fn predict_for_mat_single_row_fast(&self, fast_config: FastConfig, data: &[f64]) -> Vec<f64> {
        let mut out = vec![0.; self.num_classes()? as usize * 1];

        booster_predict_for_mat_single_row_fast(fast_config.handle, data, out.as_mut())?;

        out
    }
}

impl Drop for LightGBM {
    fn drop(&mut self) {
        unsafe { LGBM_BoosterFree(self.handle) };
    }
}
