#![allow(non_upper_case_globals)]

extern crate cuda;

use ::ffi::*;

use cuda::runtime::{CudaStream};

use std::ptr::{null_mut};

pub mod ffi;

#[derive(Clone, Copy, Debug)]
pub struct CurandError(pub curandStatus_t);

pub type CurandResult<T> = Result<T, CurandError>;

pub struct CurandGenerator {
  ptr:  curandGenerator_t,
}

impl Drop for CurandGenerator {
  fn drop(&mut self) {
    let status = unsafe { curandDestroyGenerator(self.ptr) };
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => {}
      _ => panic!(),
    }
  }
}

impl CurandGenerator {
  pub fn create() -> CurandResult<Self> {
    let mut ptr = null_mut();
    let status = unsafe { curandCreateGenerator(&mut ptr as *mut _, curandRngType_CURAND_RNG_PSEUDO_DEFAULT) };
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(CurandGenerator{ptr: ptr}),
      _ => Err(CurandError(status)),
    }
  }

  pub fn set_offset(&mut self, offset: u64) -> CurandResult<()> {
    let status = unsafe { curandSetGeneratorOffset(self.ptr, offset) };
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(()),
      _ => Err(CurandError(status)),
    }
  }

  pub fn set_stream(&mut self, stream: &mut CudaStream) -> CurandResult<()> {
    let status = unsafe { curandSetStream(self.ptr, stream.as_mut_ptr()) };
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(()),
      _ => Err(CurandError(status)),
    }
  }

  pub unsafe fn generate(&mut self, dst_dptr: *mut u32, count: usize) -> CurandResult<()> {
    let status = curandGenerate(self.ptr, dst_dptr, count);
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(()),
      _ => Err(CurandError(status)),
    }
  }

  pub unsafe fn generate_uniform(&mut self, dst_dptr: *mut f32, count: usize) -> CurandResult<()> {
    let status = curandGenerateUniform(self.ptr, dst_dptr, count);
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(()),
      _ => Err(CurandError(status)),
    }
  }

  pub unsafe fn generate_uniform64(&mut self, dst_dptr: *mut f64, count: usize) -> CurandResult<()> {
    let status = curandGenerateUniformDouble(self.ptr, dst_dptr, count);
    match status {
      curandStatus_CURAND_STATUS_SUCCESS => Ok(()),
      _ => Err(CurandError(status)),
    }
  }
}
