use rand::SeedableRng;
pub use rand::{distributions::Standard, rngs::SmallRng, Rng};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(module = "/js/demo.js")]
extern "C" {
    fn js_fill_rand(buf: &mut [u8]) -> usize;
}

pub fn new_rng() -> Option<SmallRng> {
    let mut seed = [0u8; 16];
    window()?
        .crypto()
        .ok()?
        .get_random_values_with_u8_array(&mut seed)
        .ok()?;
    Some(SmallRng::from_seed(seed))
}
