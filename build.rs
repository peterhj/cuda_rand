extern crate bindgen;

use std::env;
use std::path::{PathBuf};

fn main() {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let cuda_dir = PathBuf::from(match env::var("CUDA_HOME") {
    Ok(path) => path,
    Err(_) => "/usr/local/cuda".to_owned(),
  });

  println!("cargo:rustc-link-lib=curand");

  let curand_bindings = bindgen::Builder::default()
    .clang_arg(format!("-I{}", cuda_dir.join("include").as_os_str().to_str().unwrap()))
    .header("wrap.h")
    .whitelist_recursively(false)
    .whitelist_type("curandStatus")
    .whitelist_type("curandStatus_t")
    .whitelist_type("curandGenerator_st")
    .whitelist_type("curandGenerator_t")
    .whitelist_type("curandRngType")
    .whitelist_type("curandRngType_t")
    .whitelist_type("curandOrdering")
    .whitelist_type("curandOrdering_t")
    .whitelist_function("curandCreateGenerator")
    .whitelist_function("curandDestroyGenerator")
    .whitelist_function("curandSetStream")
    .whitelist_function("curandSetGeneratorOffset")
    .whitelist_function("curandSetGeneratorOrdering")
    .whitelist_function("curandSetPseudoRandomGeneratorSeed")
    .whitelist_function("curandGenerateSeeds")
    .whitelist_function("curandGenerate")
    .whitelist_function("curandGenerateLogNormal")
    .whitelist_function("curandGenerateLogNormalDouble")
    .whitelist_function("curandGenerateNormal")
    .whitelist_function("curandGenerateNormalDouble")
    .whitelist_function("curandGenerateUniform")
    .whitelist_function("curandGenerateUniformDouble")
    .generate()
    .expect("bindgen failed to generate curand bindings");
  curand_bindings
    .write_to_file(out_dir.join("curand_bind.rs"))
    .expect("bindgen failed to write curand bindings");
}
