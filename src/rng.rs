extern crate rand;

pub use self::rand::{distributions::Standard, rngs::SmallRng, Rng};
use rand::SeedableRng;
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(module = "/js/demo.js")]
extern "C" {
    fn js_fill_rand(buf: &mut [u8]) -> usize;
}

#[derive(Debug)]
pub enum RNGSourceError {}

pub fn new_rng() -> Result<SmallRng, RNGSourceError> {
    let mut seed = [0u8; 16];
    window()
        .unwrap()
        .crypto()
        .unwrap()
        .get_random_values_with_u8_array(&mut seed)
        .unwrap();
    Ok(SmallRng::from_seed(seed))
}
