// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sig::Signal;
use fon::{mono::Mono64, sample::Sample, Sink};
use std::{marker::PhantomData, time::Duration};

/// Frequency counter.
#[derive(Copy, Clone, Debug)]
pub struct Fc(Duration);

impl Fc {
    /// Sample frequency counter with a frequency.
    #[inline]
    pub fn freq(&self, freq: f64) -> Signal {
        let modu = Duration::new(1, 0).div_f64(freq).as_nanos();
        let nano = self.0.as_nanos();
        // Return signal between -1 and 1
        (((nano % modu) << 1) as f64 / modu as f64 - 1.0).into()
    }
}

/// A streaming synthesizer into an audio `Sink`.  Rather than implementing
/// `Stream`, which has it's own associated sample rate, `Synth` generates the
/// audio directly at the destination sample rate.
#[derive(Debug, Copy, Clone)]
pub struct Synth<S: Sample + From<Mono64>, K: Sink<S>> {
    sink: K,
    counter: Duration,
    _phantom: PhantomData<S>,
}

impl<S: Sample + From<Mono64>, K: Sink<S>> Synth<S, K> {
    /// Create a new synthesizer that feeds into an audio `Sink` (the opposite
    /// end of a stream).
    pub fn new(sink: K) -> Self {
        Self {
            sink,
            counter: Duration::new(0, 0),
            _phantom: PhantomData,
        }
    }

    /// Generate audio samples.
    /// - `count`: How many samples to stream into the audio `Sink`.
    /// - `synth`: Synthesis function to generate the audio signal.
    pub fn gen<F: FnMut(Fc) -> Signal>(&mut self, count: usize, mut synth: F) {
        let stepper = Duration::new(1, 0) / self.sink.sample_rate();
        for _ in 0..count {
            self.sink
                .sink_sample(synth(Fc(self.counter)).to_mono().into());
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
