// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::f64::consts::PI;
use fon::{mono::Mono, chan::{Channel, Ch64}, sample::Sample1};

/// A normalized and signed digital audio signal that can be routed through
/// processing components.
#[derive(Copy, Clone, Debug)]
pub struct Signal(f64);

impl Signal {
    /// Sine generator component (takes a triangle wave).
    pub fn sine(self) -> Self {
        Self((self.0 * PI).sin())
    }
}

impl From<f64> for Signal {
    fn from(signal: f64) -> Signal {
        Signal(signal)
    }
}

impl<Ch> From<Signal> for Sample1<Ch, Mono>
    where Ch: From<Ch64> + Channel
{
    fn from(signal: Signal) -> Self {
        let ch: Ch = Ch64::new(signal.0).into();
        Self::new(ch)
    }
}
