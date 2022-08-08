extern crate rand;

pub use self::rand::{Rng, SeedableRng, StdRng};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(module = "/js/demo.js")]
extern "C" {
    fn js_fill_rand(buf: &mut [u8]) -> usize;
}

#[derive(Debug)]
pub enum RNGSourceError {}

pub fn new_rng() -> Result<StdRng, RNGSourceError> {
    let mut seed = [0u8; 32];
    window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut seed)
        .unwrap();
    let seed = unsafe {
        ::std::slice::from_raw_parts::<usize>(
            seed.as_ptr() as *const usize,
            32 / ::std::mem::size_of::<usize>(),
        )
    };
    Ok(StdRng::from_seed(seed))
}
