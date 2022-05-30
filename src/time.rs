pub use std::time::Duration;

#[derive(Copy, Clone)]
pub struct Instant {
    now: f64,
}

impl Instant {
    pub fn now() -> Instant {
        unsafe {
            Instant {
                now: web_sys::window().unwrap().performance().unwrap().now(),
            }
        }
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        let diff = (self.now - earlier.now) * 0.001;
        let secs = diff.floor() as u64;
        let nano = (diff.fract() * 1e9) as u32;
        Duration::new(secs, nano)
    }
    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.clone())
    }
}
