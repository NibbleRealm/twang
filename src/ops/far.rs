#[cfg(not(test))]
use crate::math::Libm;

use fon::chan::{Ch32, Channel};

/// Limit amplitude of a sample with another.
#[derive(Debug, Clone, Copy, Default)]
pub struct Far;

impl Far {
    /// Get next sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, other: Ch32) -> Ch32 {
        let other = other.to_f32().abs();
        let input = input.to_f32();
        if input < 0.0 {
            Ch32::from(input.min(-other))
        } else {
            Ch32::from(input.max(other))
        }
    }
}
