use crate::bindings::{FastConfigHandle, LGBM_FastConfigFree};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct FastConfig<'a> {
    pub(crate) handle: FastConfigHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> FastConfig<'a> {
    pub(crate) fn new(handle: FastConfigHandle) -> FastConfig<'a> {
        FastConfig { handle, _phantom: PhantomData }
    }
}

impl<'a> Drop for FastConfig<'a> {
    fn drop(&mut self) {
        unsafe {
            LGBM_FastConfigFree(self.handle);
        }
    }
}
