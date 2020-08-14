// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Digital audio signal.

use fon::{
    chan::{Ch64, Channel},
    mono::Mono,
    sample::Sample1,
};
use std::f64::consts::PI;

/// A signed digital audio signal that can be routed through processing
/// components.  This differs from `Mono64` in that the values are not clamped
/// between -1 and 1.
#[derive(Copy, Clone, Debug)]
pub struct Signal(f64);

impl Signal {
    /// Sine generator component - takes a sawtooth (`Fc`) wave.
    pub fn sine(self) -> Self {
        Self((self.0 * PI).sin())
    }

    /// Amplify signal.
    pub fn amp(self, volume: f64) -> Self {
        Self(self.0 * volume)
    }

    /*    /// Square generator component - takes a sawtooth (`Fc`) wave.
    pub fn square(self, volume: f64) -> Self {
        Self(self.0.signum() * volume)
    }*/

    /// Signal inversion (negation).
    pub fn invert(self) -> Self {
        Self(-self.0)
    }
}

impl From<f64> for Signal {
    fn from(signal: f64) -> Signal {
        Signal(signal)
    }
}

impl From<Signal> for f64 {
    fn from(signal: Signal) -> f64 {
        signal.0
    }
}

impl<Ch> From<Signal> for Sample1<Ch, Mono>
where
    Ch: From<Ch64> + Channel,
{
    fn from(signal: Signal) -> Self {
        let power: f64 = signal.into();
        let ch: Ch = Ch64::new(power.min(1.0).max(-1.0)).into();
        Self::new(ch)
    }
}
