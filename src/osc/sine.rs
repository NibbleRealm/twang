#[cfg(not(test))]
use crate::math::Libm;

use fon::chan::Ch32;

/// Sine wave generator.
#[derive(Default, Clone, Copy, Debug)]
pub struct Sine(f32);

impl Sine {
    /// Create a new sine wave generator.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the next sample from the oscillator without progressing oscillator.
    #[inline(always)]
    pub fn peek(&mut self) -> Ch32 {
        self.0.cos().into()
    }

    /// Get the next sample from this oscillator.
    #[inline(always)]
    pub fn step(&mut self, hz: f32) -> Ch32 {
        let out = self.peek();
        self.0 = (self.0 + super::TAU * super::SAMPLE_PERIOD * hz) % super::TAU;
        out
    }

    /// Get the phase-shifted sample from this oscillator.
    #[inline(always)]
    pub fn phase(&mut self, hz: f32, shift: Ch32) -> Ch32 {
        use super::TAU;

        let original = self.0;
        let shift = if f32::from(shift) < 0.0 {
            1.0 + f32::from(shift)
        } else {
            f32::from(shift)
        };
        self.0 = (original + TAU * (super::SAMPLE_PERIOD * hz + shift)) % TAU;
        let out = self.peek();
        self.0 = (original + TAU * super::SAMPLE_PERIOD * hz) % TAU;
        out
    }

    /// Phase shift this oscillator.
    #[inline(always)]
    pub fn shift(&mut self, shift: Ch32) {
        let shift = if f32::from(shift) < 0.0 {
            1.0 + f32::from(shift)
        } else {
            f32::from(shift)
        };
        self.0 = (self.0 + super::TAU * shift) % super::TAU;
    }
}
