use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use wasm_bindgen::prelude::wasm_bindgen;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn sum(numbers: &[i32]) -> i32 {
    numbers.par_iter().sum()
}
