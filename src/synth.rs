// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sig::Signal;
use fon::{mono::Mono64, sample::Sample, Audio, Hz};
use std::time::Duration;

/// Frequency counter.
#[derive(Copy, Clone, Debug)]
pub struct Fc(Duration);

impl Fc {
    /// Sample frequency counter with a frequency.
    #[inline]
    pub fn freq<H: Into<Hz>>(&self, freq: H) -> Signal {
        let modu = Duration::new(1, 0).div_f64(freq.into().0).as_nanos();
        let nano = self.0.as_nanos();
        // Return signal between -1 and 1
        (((nano % modu) << 1) as f64 / modu as f64 - 1.0).into()
    }
}

/// A synthesizer for an `Audio` buffer.
#[derive(Debug, Copy, Clone, Default)]
pub struct Synth {
    counter: Duration,
}

impl Synth {
    /// Create a new synthesizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate audio samples to fill a buffer.
    pub fn gen<S: Sample + From<Mono64>, F: FnMut(Fc) -> Signal>(
        &mut self,
        audio: &mut Audio<S>,
        mut f: F,
    ) {
        let stepper = Duration::new(1, 0) / audio.sample_rate();
        for sample in audio.iter_mut() {
            *sample = f(Fc(self.counter)).to_mono().into();
            self.counter += stepper;
        }
    }
}

/// Trait for synthesizing multiple sounds together.
pub trait Mix {
    /// Add two signals together.
    fn mix(self) -> Signal;
}

impl<I: IntoIterator<Item = Signal>> Mix for I {
    fn mix(self) -> Signal {
        self.into_iter().map(f64::from).sum::<f64>().into()
    }
}
