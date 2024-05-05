use fon::{
    chan::{Ch32, Channel},
    Sink,
};

use crate::tree::{Chunk, Wave};

/// A streaming synthesizer
#[derive(Debug)]
pub struct Synth<W> {
    wave: W,
    elapsed: u64,
    cursor: usize,
    chunk: Chunk,
}

impl<W> Synth<W>
where
    W: Wave,
{
    /// Create a new synthesizer based on a waveform
    pub fn new(wave: W) -> Self {
        Self {
            wave,
            elapsed: 0,
            cursor: 32,
            chunk: Chunk([0.0; 32]),
        }
    }

    /// Run synthesis with user parameters, streaming output into the provided
    /// [`Sink`]
    pub fn stream<Ch, K, const N: usize>(&mut self, mut sink: K, params: &[f32])
    where
        Ch: Channel + From<Ch32>,
        K: Sink<Ch, N>,
    {
        let sample_rate: u32 = sink.sample_rate().into();
        let synth_iter = SynthIter(self, sample_rate, params);

        sink.sink_with(&mut synth_iter.map(|x| x.to()));
    }

    fn synthesize(&mut self, sample_rate: u32, params: &[f32]) -> f32 {
        if self.cursor == 32 {
            self.cursor = 0;

            let interval = 32_000_000_000 / u64::from(sample_rate);

            self.chunk = self.wave.synthesize(self.elapsed, interval, params);
            self.elapsed += interval;
        }

        let cursor = self.cursor;

        self.cursor += 1;
        self.chunk.0[cursor]
    }
}

struct SynthIter<'a, 'b, W>(&'a mut Synth<W>, u32, &'b [f32]);

impl<W> Iterator for SynthIter<'_, '_, W>
where
    W: Wave,
{
    type Item = fon::Frame<Ch32, 1>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(synth, sample_rate, params) = self;

        Some(synth.synthesize(*sample_rate, params).into())
    }
}
