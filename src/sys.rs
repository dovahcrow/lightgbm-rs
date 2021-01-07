#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::errors::LightGBMError;
use failure::Error;
use fehler::{throw, throws};
use libc::{c_int, c_void};
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

const C_API_IS_ROW_MAJOR: c_int = 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum PredictType {
    Normal = C_API_PREDICT_NORMAL as i32,
    RawScore = C_API_PREDICT_RAW_SCORE as i32,
    LeafIndex = C_API_PREDICT_LEAF_INDEX as i32,
    Contrib = C_API_PREDICT_CONTRIB as i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum DType {
    Float32 = C_API_DTYPE_FLOAT32 as i32,
    Float64 = C_API_DTYPE_FLOAT64 as i32,
    Int32 = C_API_DTYPE_INT32 as i32,
    Int64 = C_API_DTYPE_INT64 as i32,
}

#[throws(Error)]
pub fn booster_create_from_modelfile(filename: &str) -> (BoosterHandle, i32) {
    let cfilename = CString::new(filename)?;
    let mut ntrees: c_int = -1;
    let mut handle = null_mut() as BoosterHandle;

    if unsafe { LGBM_BoosterCreateFromModelfile(cfilename.as_ptr(), &mut ntrees, &mut handle) } != 0 {
        throw!(get_last_error());
    };

    (handle, ntrees)
}

#[throws(Error)]
pub fn booster_get_num_classes(handle: BoosterHandle) -> i32 {
    let mut num = 0;

    if unsafe { LGBM_BoosterGetNumClasses(handle, &mut num) } != 0 {
        throw!(get_last_error());
    }

    num
}

#[throws(Error)]
pub fn booster_predict_for_mat_single_row(
    handle: BoosterHandle,
    data: &[f64],
    predict_type: PredictType,
    start_iteration: i32,
    num_iteration: i32,
    data_type: DType,
    params: &str,
    out: &mut [f64],
) {
    let mut outlen: i64 = 0;
    let params = CString::new(params)?;

    let ret = unsafe {
        LGBM_BoosterPredictForMatSingleRow(
            handle,
            data.as_ptr() as *const c_void,
            DType::Float64 as i32,
            data.len() as i32,
            C_API_IS_ROW_MAJOR,
            predict_type as i32,
            start_iteration,
            num_iteration,
            params.as_ptr(),
            &mut outlen,
            out.as_mut_ptr(),
        )
    };

    if ret != 0 {
        throw!(get_last_error());
    }
}

#[throws(Error)]
pub fn booster_predict_for_mat_single_row_fast_init(
    handle: BoosterHandle,
    predict_type: PredictType,
    start_iteration: i32,
    num_iteration: i32,
    data_type: DType,
    ncol: i32,
    params: &str,
) -> FastConfigHandle {
    let mut out = null_mut();

    let params = CString::new(params)?;
    let ret = unsafe {
        LGBM_BoosterPredictForMatSingleRowFastInit(
            handle,
            predict_type as c_int,
            start_iteration,
            num_iteration,
            data_type as i32,
            ncol,
            params.as_ptr(),
            &mut out,
        )
    };

    if ret != 0 {
        throw!(get_last_error());
    }
    out
}

#[throws(Error)]
pub fn booster_predict_for_mat_single_row_fast(handle: FastConfigHandle, data: &[f64], out: &mut [f64]) {
    let mut outlen: i64 = 0;
    let ret = unsafe { LGBM_BoosterPredictForMatSingleRowFast(handle, data.as_ptr() as *const c_void, &mut outlen, out.as_mut_ptr()) };
    if ret != 0 {
        throw!(get_last_error());
    }
}

pub fn get_last_error() -> LightGBMError {
    let cs = unsafe { CStr::from_ptr(LGBM_GetLastError()) };

    LightGBMError::CError(cs.to_string_lossy().to_owned().to_string())
}
