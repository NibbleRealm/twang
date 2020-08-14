// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::Generator;
use core::time::Duration;
use fon::{mono::Mono64, Hz};

/// Sawtooth wave generator.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct Saw {
    hertz: Hz,
    value: f64,
}

impl Saw {
    /// Create a sawtooth wave generator.
    pub fn new(hertz: Hz) -> Self {
        let value = -1.0;
        Self { hertz, value }
    }

    /// Get the pitch of the sound.
    pub fn pitch(&self) -> Hz {
        self.hertz
    }

    /// Change the pitch of the saw wave.    
    pub fn set_pitch(&mut self, pitch: Hz) {
        self.hertz = pitch;
    }
}

impl Generator for Saw {
    fn sample(&mut self, duration: Duration) -> Mono64 {
        self.value = (self.value + duration.as_secs_f64() * self.hertz.0) % 1.0;
        Mono64::new(self.value * 2.0 - 1.0)
    }
}
