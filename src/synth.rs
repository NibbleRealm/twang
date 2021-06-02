// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::sig::Signal;
use fon::{Stream, Audio};
use fon::chan::{Channel, Ch64, Ch32};
use std::{borrow::Borrow, fmt::Debug, time::Duration};

/// Frequency counter.
#[derive(Copy, Clone, Debug)]
pub struct Fc(Duration);

impl Fc {
    /// Sample frequency counter with a frequency.
    #[inline(always)]
    pub fn freq(&self, freq: f64) -> Signal {
        let modu = Duration::new(1, 0).div_f64(freq).as_nanos();
        let nano = self.0.as_nanos();
        // Return signal between -1 and 1
        (((nano % modu) << 1) as f64 / modu as f64 - 1.0).into()
    }

    /// Create an oscillator based on this frequency counter.  Frequency should
    /// always be more than 0.000_000_001 hertz (panic in debug mode).
    #[inline(always)]
    pub fn osc(&self, freq: f32) -> Ch32 {
        debug_assert!(freq > 0.000_000_001);

        let period: u64 = (1_000_000_000.0 / freq) as u64;

        // Modulo duration by period.
        let secs = self.0.as_secs() % period;
        let nanos = self.0.subsec_nanos();
        let dur = secs * 1_000_000_000 + u64::from(nanos);
        let time = (dur % period) as f32 * 0.000_000_001;

        Ch32::from(time)
    }
}

/// A streaming synthesizer.  Implements [`Stream`](fon::Stream).
pub struct Synth<T: Debug> {
    params: T,
    synthfn: fn(&mut T, Fc) -> Signal,
    counter: Duration,
}

impl<T: Debug> Debug for Synth<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Debug> Synth<T> {
    /// Create a new streaming synthesizer.
    #[inline(always)]
    pub fn new(params: T, synth: fn(&mut T, Fc) -> Signal) -> Self {
        Self {
            params,
            synthfn: synth,
            counter: Duration::default(),
        }
    }

    /// Get the parameters of the synthesizer.
    pub fn params(&mut self) -> &mut T {
        &mut self.params
    }
}

impl<T: Debug> Stream<Ch64, 1> for Synth<T> {
    fn sample_rate(&self) -> Option<u32> {
        // Synthesizer has no fixed sample rate.
        None
    }

    fn extend<C: Channel, const N: usize>(
        &mut self,
        buffer: &mut Audio<C, N>,
        len: usize
    ) where
        C: From<Ch64>,
    {
        let step = Duration::new(1, 0) / Audio::sample_rate(buffer);
        let synth = (0..len).map(|_| {
            let frame =
                (self.synthfn)(&mut self.params, Fc(self.counter)).to_mono();
            self.counter += step;
            frame.to()
        });
        buffer.sink(synth);
    }
}

/// Trait for synthesizing multiple sounds together.
///
/// This works on arrays, slices, and iterators over either `Signal` or
/// `&Signal`.
pub trait Mix {
    /// Add multiple signals together.
    fn mix(self) -> Signal;
}

impl<B: Borrow<Signal>, I: IntoIterator<Item = B>> Mix for I {
    #[inline(always)]
    fn mix(self) -> Signal {
        self.into_iter()
            .map(|a| f64::from(*a.borrow()))
            .sum::<f64>()
            .into()
    }
}
