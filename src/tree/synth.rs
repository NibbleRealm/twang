use alloc::{vec, vec::Vec};

use fon::{
    chan::{Ch32, Channel},
    Sink,
};

use crate::tree::{consts, Chunk, Parameters, Params, Wave};

#[allow(missing_debug_implementations)]
pub struct Data<'a> {
    /// Reference to state slice at index
    pub(crate) state: &'a mut [u32],
    /// 1 hertz sample steps for chunk (0, 1/32, 2/32, etc.)
    pub(crate) sample_steps: &'a [f32; 32],
    /// User parameters
    pub(crate) params: &'a mut dyn Parameters,
    /// 1 hertz chunk step (1; 32 samples)
    pub(crate) chunk_step: f32,
}

/// A streaming synthesizer
#[derive(Debug)]
pub struct Synth<W, const N: usize>
where
    W: Wave,
{
    chunk: Chunk,
    cursor: usize,
    state: Vec<u32>,
    wave: W,
    params: Params<N>,
}

impl<W, const N: usize> Synth<W, N>
where
    W: Wave,
{
    /// Create a new synthesizer based on a waveform.
    pub fn new(wave: W, params: [f32; N]) -> Self {
        Self {
            wave,
            cursor: 32,
            chunk: Chunk([0.0; 32]),
            state: vec![0; W::STATE_LEN],
            params: Params::new(params),
        }
    }

    /// Get a mutable reference to the parameters
    pub fn params(&mut self) -> &mut [f32; N] {
        self.params.get_mut()
    }

    /// Run synthesis with user parameters, streaming output into the provided
    /// [`Sink`].
    pub fn stream<Ch, const S: usize>(&mut self, mut sink: impl Sink<Ch, S>)
    where
        Ch: Channel + From<Ch32>,
    {
        let chunk_step: f32 =
            (sink.sample_rate().get() as f32 * consts::FRAC_32[1]).recip();
        let mut sample_steps = [chunk_step; 32];

        sample_steps
            .iter_mut()
            .zip(consts::FRAC_32.iter())
            .for_each(|(sample, &mul)| {
                *sample *= mul;
            });

        let synth_iter = SynthIter(self, chunk_step, sample_steps);

        sink.sink_with(&mut synth_iter.map(|x| x.to()));
    }

    fn synthesize(&mut self, chunk_step: f32, sample_steps: &[f32; 32]) -> f32 {
        if self.cursor == 32 {
            let mut data = Data {
                state: self.state.as_mut_slice(),
                params: &mut self.params,
                sample_steps,
                chunk_step,
            };

            self.cursor = 0;
            self.chunk = self.wave.synthesize(&mut data);
            self.params.mark_old();
        }

        let cursor = self.cursor;

        self.cursor += 1;
        self.chunk.0[cursor]
    }
}

struct SynthIter<'a, W, const N: usize>(&'a mut Synth<W, N>, f32, [f32; 32])
where
    W: Wave;

impl<W, const N: usize> Iterator for SynthIter<'_, W, N>
where
    W: Wave,
{
    type Item = fon::Frame<Ch32, 1>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(synth, chunk_step, sample_steps) = self;

        Some(synth.synthesize(*chunk_step, sample_steps).into())
    }
}
