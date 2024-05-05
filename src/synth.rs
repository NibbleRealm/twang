#![allow(clippy::all)]

use alloc::boxed::Box;
use core::fmt::{Debug, Error, Formatter};
use fon::chan::{Ch32, Channel};
use fon::{Frame, Sink};

/// A synthesizer stream.
pub struct Synth<S, const CH: usize>(
    S,
    Box<dyn FnMut(&mut S, Frame<Ch32, CH>) -> Frame<Ch32, CH>>,
);

impl<S, const CH: usize> Debug for Synth<S, CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Synth")
    }
}

impl<S, const CH: usize> Synth<S, CH> {
    /// Create a new synthesizer function.
    pub fn new<F>(s: S, f: F) -> Self
    where
        F: 'static + FnMut(&mut S, Frame<Ch32, CH>) -> Frame<Ch32, CH>,
    {
        Self(s, Box::new(f))
    }

    /// Stream synthesized samples into a [`Sink`].
    pub fn stream<Chan: Channel, K>(&mut self, mut sink: K)
    where
        K: Sink<Chan, CH>,
        Chan: From<Ch32>,
    {
        let sample_rate: u32 = sink.sample_rate().into();
        let synth_iter = SynthIter(self, sample_rate);
        sink.sink_with(&mut synth_iter.map(|x| x.to()));
    }
}

struct SynthIter<'a, S, const CH: usize>(&'a mut Synth<S, CH>, u32);

impl<S, const CH: usize> Iterator for SynthIter<'_, S, CH> {
    type Item = Frame<Ch32, CH>;

    fn next(&mut self) -> Option<Self::Item> {
        assert_eq!(self.1, 48_000);
        Some(self.0 .1(&mut self.0 .0, Default::default()))
    }
}
