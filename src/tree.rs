macro_rules! const_postfix_waveform {
    () => {
        const fn sine(self) -> Sine<Self> {
            Sine(self)
        }
    };
    ($type:ty) => {
        impl $type {
            const_postfix_waveform!();
        }
    };
    ($type:ty, $($generic:ident),+) => {
        impl<$($generic),+> $type {
            const_postfix_waveform!();
        }
    };
}

const_postfix_waveform!(Hz);
const_postfix_waveform!(Line);
const_postfix_waveform!(Sine<T>, T);

use core::{time::Duration, f32::consts};

#[derive(Debug)]
pub struct Chunk([f32; 32]);

impl Chunk {
    #[inline(always)]
    fn for_each_sample(&mut self, f: impl FnMut(&mut f32)) {
        self.0.iter_mut().for_each(f);
    }

    #[inline(always)]
    fn offset(&mut self, amt: f32) {
        self.for_each_sample(|sample| *sample += amt);
    }

    #[inline(always)]
    fn amplify(&mut self, amt: f32) {
        self.for_each_sample(|sample| *sample *= amt);
    }

    #[inline(always)]
    fn cosine(&mut self) {
        self.for_each_sample(|sample| *sample = libm::cosf(*sample));
    }

    #[inline(always)]
    fn invert(&mut self) {
        self.for_each_sample(|sample| *sample = -*sample);
    }
}

pub trait Wave {
    /// Synthesize a chunk of audio.
    ///
    /// - `elapsed` is nanoseconds since the start of synthesis (up to about
    ///   1169 years)
    /// - `interval` is nanoseconds in the chunk's interval
    #[must_use]
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk;
}

impl<T> Wave for &T
where
    T: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk {
        (**self).synthesize(elapsed, interval)
    }
}

/// Constant signal
#[derive(Debug)]
pub struct Line(pub f32);

impl Wave for Line {
    fn synthesize(&self, _elapsed: u64, _interval: u64) -> Chunk {
        Chunk([self.0; 32])
    }
}

/// Fixed frequency phase counter
///
/// Produces a sawtooth wave
#[derive(Debug)]
pub struct Hz(pub f32);

impl Wave for Hz {
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk {
        let hz_32 = (self.0 * 32.0) as u64;
        let phase = 32_000_000_000 / hz_32;
        let offset = elapsed % phase;
        let mut i = 0;
        let mut chunk = Chunk([0.0; 32]);

        chunk.for_each_sample(|sample| {
            let place = i * interval / 32 + offset;

            *sample = (place as f32 / phase as f32) % 1.0;
            i += 1;
        });
        chunk.amplify(-2.0);
        chunk.offset(1.0);
        chunk
    }
}

/// Sine wave
///
/// Takes phase (-1 to 1) as input
#[derive(Debug)]
pub struct Sine<I>(pub I);

impl<I> Wave for Sine<I>
where
    I: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk {
        let mut chunk = self.0.synthesize(elapsed, interval);

        chunk.amplify(consts::PI);
        chunk.cosine();
        chunk.invert();
        chunk
    }
}

#[derive(Debug)]
pub struct Synth<W> {
    wave: W,
    elapsed: u64,
    cursor: usize,
    chunk: Chunk,
}

impl<W> Synth<W>
where W: Wave
{
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
        Ch: fon::chan::Channel + From<fon::chan::Ch32>,
        K: fon::Sink<Ch, N>,
    {
        let sample_rate: u32 = sink.sample_rate().into();
        let synth_iter = SynthIter(self, sample_rate);

        sink.sink_with(&mut synth_iter.map(|x| x.to()));
    }

    fn synthesize(&mut self, sample_rate: u32) -> f32 {
        if self.cursor == 32 {
            self.cursor = 0;

            let interval = 32_000_000_000 / u64::from(sample_rate);

            self.chunk = self.wave.synthesize(self.elapsed, interval);
            self.elapsed += interval;
        }

        let cursor = self.cursor;

        self.cursor += 1;
        self.chunk.0[cursor]
    }
}

struct SynthIter<'a, W>(&'a mut Synth<W>, u32);

impl<W> Iterator for SynthIter<'_, W>
where W: Wave
{
    type Item = fon::Frame<fon::chan::Ch32, 1>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(synth, sample_rate) = self;

        Some(synth.synthesize(*sample_rate).into())
    }
}
