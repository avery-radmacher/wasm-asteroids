extern crate rand;

pub use self::rand::{Rng, SeedableRng, StdRng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/demo.js")]
extern "C" {
    fn js_fill_rand(ptr: *mut u8, len: usize) -> usize;
}

#[derive(Debug)]
pub enum RNGSourceError {
    RangeError,
    QuotaError,
    UnknownError,
}

pub fn fill_random(buf: &mut [u8]) -> Result<(), RNGSourceError> {
    for (i, b) in buf.iter_mut().enumerate() {
        *b = i as u8;
    }
    //let rv = unsafe { js_fill_rand(buf.as_mut_ptr(), buf.len()) };
    // match rv {
    //     0 => Ok(()),
    //     1 => Err(RNGSourceError::RangeError),
    //     2 => Err(RNGSourceError::QuotaError),
    //     _ => Err(RNGSourceError::UnknownError),
    // }
    Ok(())
}

pub fn new_rng() -> Result<StdRng, RNGSourceError> {
    let mut seed = [0u8; 32];
    fill_random(&mut seed)?;
    let seed = unsafe {
        ::std::slice::from_raw_parts::<usize>(
            seed.as_ptr() as *const usize,
            32 / ::std::mem::size_of::<usize>(),
        )
    };
    Ok(StdRng::from_seed(seed))
}
