use rand::SeedableRng;
pub use rand::{distributions::Standard, rngs::SmallRng, Rng};
use web_sys::window;

pub fn new_rng() -> Option<SmallRng> {
    let mut seed = [0u8; 16];
    window()?
        .crypto()
        .ok()?
        .get_random_values_with_u8_array(&mut seed)
        .ok()?;
    Some(SmallRng::from_seed(seed))
}
