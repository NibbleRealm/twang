// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::time::Duration;
use fon::{Hz, chan::Ch64};
use crate::sig::Signal;

/// Frequency counter.
#[derive(Copy, Clone, Debug)]
pub struct Fc {
    counter: Duration,
    stepper: Duration,
}

impl Fc {
    /// Create a new frequency counter.
    #[inline]
    pub fn new(sample_freq: u32) -> Self {
        let counter = Duration::new(0, 0);
        let stepper = Duration::new(1, 0) / sample_freq;
        Self {
            counter,
            stepper,
        }
    }

    /// Advance to the next sample.
    #[inline]
    pub fn step(&mut self) {
        self.counter += self.stepper
    }

    /// Sample frequency counter with a frequency.
    #[inline]
    pub fn freq<H: Into<Hz>>(&self, freq: H) -> Signal {
        let modu = Duration::new(1, 0).div_f64(freq.into().0).as_nanos();
        let nano = self.counter.as_nanos();
        // Return signal between -1 and 1
        (((nano % modu) << 1) as f64 / modu as f64 - 1.0).into()
    }
}
