use crate::quiet::Quiet;
use crate::Sample;

use rand::distributions::{Uniform, Distribution};
use rand_pcg::Mcg128Xsl64;

/// White Noise Sampler.
pub struct White {
    dist: Uniform<f64>,
    rng: Mcg128Xsl64,
    sampler: Quiet,
}

impl White {
    /// Create a new White Noise Sampler.
    pub fn new(hz: Option<f64>) -> Self {
        Self {
            dist: Uniform::new_inclusive(-1.0, 1.0),
            rng: Mcg128Xsl64::new(0xcafef00dd15ea5e5 /* default from docs */),
            sampler: Quiet::new(hz),
        }
    }

    fn sample(&mut self) -> f64 {
        self.dist.sample(&mut self.rng)
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
