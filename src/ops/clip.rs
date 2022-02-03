// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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
