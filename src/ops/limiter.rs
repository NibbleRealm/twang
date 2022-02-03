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

/// Soft clipping / limiting.
///
/// A "limit" channel controls the maximum amplitude of the "input" channel.
#[derive(Debug, Clone, Copy, Default)]
pub struct Limiter;

impl Limiter {
    /// Get next limited sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32, limit: Ch32) -> Ch32 {
        let limit = limit.to_f32();
        let volume = 1.0 / limit;
        if volume.is_nan() {
            return Ch32::default();
        }
        Ch32::from(
            limit * (2.0 / (1.0 + (input.to_f32() * -volume).exp()) - 1.0)
                / (2.0 / (1.0 + (-volume).exp()) - 1.0),
        )
    }
}
