// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
