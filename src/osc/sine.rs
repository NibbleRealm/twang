// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
    pub fn next(&mut self, hz: f32) -> Ch32 {
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
