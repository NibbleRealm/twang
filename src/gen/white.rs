use super::Generator;
use std::num::Wrapping;
use std::time::Duration;

const SEQUENCE: u64 = 0xb5ad4eceda1ce2a9;

/// White Noise Generator using Middle Square Weyl Sequence PRNG.
pub struct White {
    x: Wrapping<u64>,
    w: Wrapping<u64>,
}

impl White {
    /// Create a new White Noise Sampler.
    pub fn new() -> Self {
        Self {
            x: Wrapping(0),
            w: Wrapping(0),
        }
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
