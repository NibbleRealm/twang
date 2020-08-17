// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::num::Wrapping;
use crate::sig::Signal;

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
    pub fn new() -> Self {
        Self::default()
    }

    /// Get next sample of white noise.
    pub fn noise(&mut self) -> Signal {
        // msws (Middle Square Weyl Sequence) algorithm
        self.x *= self.x;
        self.w += Wrapping(SEQUENCE);
        self.x += self.w;
        self.x = (self.x >> 32) | (self.x << 32);
        Signal::from(
            ((self.x.0 as i32) as f64 + 0.5) * (i32::MAX as f64 + 0.5).recip(),
        )
    }
}
