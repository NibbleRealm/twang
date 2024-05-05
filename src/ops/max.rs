use fon::chan::{Ch32, Channel};

/// Maximum value of two samples (warning: -0.25 > -0.5, if you want maximum
/// amplitude; -0.5 > -0.25 use [`Far`](crate::ops::Far)).
#[derive(Debug, Clone, Copy, Default)]
pub struct Max;

impl Max {
    /// Get next sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, other: Ch32) -> Ch32 {
        Ch32::from(input.to_f32().max(other.to_f32()))
    }
}
