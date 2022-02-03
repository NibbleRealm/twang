// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
