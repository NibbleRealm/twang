use crate::tree::{Chunk, Data, Wave, consts};

/// Phase oscillator (sawtooth wave)
///
/// Takes frequency (non-zero) as input
///
/// This is the most basic oscillator, which all other oscillators must depend
/// on for their phase.
#[derive(Debug)]
pub struct Osc<I>(pub I);

impl<I> Wave for Osc<I>
where
    I: Wave,
{
    const STATE_LEN: usize = I::STATE_LEN + 1;

    fn synthesize(&self, data: &mut Data<'_>) -> Chunk {
        let mut i = 0;
        let mut phase = f32::from_bits(data.state[0]);
        let chunk = self.0
            .synthesize(data)
            .for_each_sample(|sample| {
                let frequency = *sample;

                *sample = phase % 1.0;
                phase += data.chunk_step * frequency * consts::FRAC_32[1];
                i += 1;
            })
            .gain(-2.0)
            .offset(1.0);

        data.state[0] = (phase % 1.0).to_bits();
        chunk
    }
}
