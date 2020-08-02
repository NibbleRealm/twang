use std::num::Wrapping;
use crate::quiet::Quiet;
use crate::Sample;

const SEQUENCE: u64 = 0xb5ad4eceda1ce2a9;

/// White Noise Sampler.  Uses the Middle Square Weyl Sequence PRNG.
pub struct White {
    sampler: Quiet,
    x: Wrapping<u64>,
    w: Wrapping<u64>,
}

impl White {
    /// Create a new White Noise Sampler.
    pub fn new(hz: Option<f64>) -> Self {
        Self {
            x: Wrapping(0),
            w: Wrapping(0),
            sampler: Quiet::new(hz),
        }
    }

    fn sample(&mut self) -> f64 {
        // msws (Middle Square Weyl Sequence) algorithm
        self.x *= self.x;
        self.w += Wrapping(SEQUENCE);
        self.x += self.w;
        self.x = (self.x >> 32) | (self.x << 32);
        ((self.x.0 as u32) as f64) / u32::MAX as f64
    }
}

impl Iterator for White {
    type Item = Sample;

    fn next(&mut self) -> Option<Sample> {
        let mut sample = self.sampler.next().unwrap();
        sample.v = self.sample();
        Some(sample)
    }
}
