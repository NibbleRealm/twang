// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::num::Wrapping;
use fon::chan::Ch24;

const SEQUENCE: u64 = 0xb5ad4eceda1ce2a9;

/// White Noise Generator using Middle Square Weyl Sequence PRNG.
#[derive(Default, Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct White {
    x: Wrapping<u64>,
    w: Wrapping<u64>,
}

impl White {
    /// Create a new White Noise Sampler.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get next sample from the noise generator.
    #[inline(always)]
    pub fn step(&mut self) -> fon::chan::Ch32 {
        // msws (Middle Square Weyl Sequence) algorithm
        self.x *= self.x;
        self.w += Wrapping(SEQUENCE);
        self.x += self.w;
        self.x = (self.x >> 32) | (self.x << 32);
        Ch24::new((self.x.0 as i32) >> 8).into()
    }
}
