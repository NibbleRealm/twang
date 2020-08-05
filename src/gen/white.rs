use super::Generator;
use core::num::Wrapping;
use core::time::Duration;

const SEQUENCE: u64 = 0xb5ad4eceda1ce2a9;

/// White Noise Generator using Middle Square Weyl Sequence PRNG.
#[derive(Default, Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct White {
    x: Wrapping<u64>,
    w: Wrapping<u64>,
}

impl White {
    /// Create a new White Noise Sampler.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Generator for White {
    fn sample(&mut self, _duration: Duration) -> f64 {
        // msws (Middle Square Weyl Sequence) algorithm
        self.x *= self.x;
        self.w += Wrapping(SEQUENCE);
        self.x += self.w;
        self.x = (self.x >> 32) | (self.x << 32);
        ((self.x.0 as i32) as f64 + 0.5) * (i32::MAX as f64 + 0.5).recip()
    }
}
