#[cfg(not(test))]
use crate::math::Libm;

use fon::chan::{Ch32, Channel};

/// Control the gain of the input the amplitude of another sample.
#[derive(Debug, Clone, Copy, Default)]
pub struct Gain;

impl Gain {
    /// Get next sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, gain: Ch32) -> Ch32 {
        Ch32::from(input.to_f32() * gain.to_f32().abs())
    }
}
