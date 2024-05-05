use fon::chan::{Ch32, Channel};

/// Minimum value of two samples (warning: -0.5 < -0.25, if you want minimum
/// amplitude; -0.25 < -0.5 use [`Near`](crate::ops::Near)).
#[derive(Debug, Clone, Copy, Default)]
pub struct Min;

impl Min {
    /// Get next sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, other: Ch32) -> Ch32 {
        Ch32::from(input.to_f32().min(other.to_f32()))
    }
}
