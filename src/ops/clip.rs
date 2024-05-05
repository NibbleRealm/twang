#[cfg(not(test))]
use crate::math::Libm;

use fon::chan::{Ch32, Channel};

/// Hard clipping / limiting.
///
/// A "limit" channel controls the maximum amplitude of the "input" channel.
#[derive(Debug, Clone, Copy, Default)]
pub struct Clip;

impl Clip {
    /// Get next clipped sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, limit: Ch32) -> Ch32 {
        let limit = limit.to_f32().abs();
        Ch32::from(input.to_f32().clamp(-limit, limit))
    }
}
