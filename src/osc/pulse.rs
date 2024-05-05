#[cfg(not(test))]
use crate::math::Libm;

use fon::chan::Ch32;

/// Pulse wave generator.
///
/// The duty cycle of the pulse wave is controlled by a side-chain channel.
/// A value of 0.0 produces a square wave.  -1.0 produces a constant signal of
/// -1.0, and +1.0 produces a constant signal of +1.0.
#[derive(Default, Clone, Copy, Debug)]
pub struct Pulse(f32);

impl Pulse {
    /// Create a new sine wave generator.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the next sample from the oscillator without progressing oscillator.
    #[inline(always)]
    pub fn peek(&mut self, duty: Ch32) -> Ch32 {
        ((f32::from(duty) + 1.0) - self.0).signum().into()
    }

    /// Get the next sample from this oscillator.
    #[inline(always)]
    pub fn step(&mut self, hz: f32, duty: Ch32) -> Ch32 {
        let out = self.peek(duty);
        self.0 = (self.0 + 2.0 * super::SAMPLE_PERIOD * hz) % 2.0;
        out
    }

    /// Get the phase-shifted sample from this oscillator.
    #[inline(always)]
    pub fn phase(&mut self, hz: f32, duty: Ch32, shift: Ch32) -> Ch32 {
        let original = self.0;
        let shift = if f32::from(shift) < 0.0 {
            1.0 + f32::from(shift)
        } else {
            f32::from(shift)
        };
        self.0 = (original + 2.0 * (super::SAMPLE_PERIOD * hz + shift)) % 2.0;
        let out = self.peek(duty);
        self.0 = (original + 2.0 * super::SAMPLE_PERIOD * hz) % 2.0;
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
        self.0 = (self.0 + 2.0 * shift) % 2.0;
    }
}
