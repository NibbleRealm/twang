use crate::tree::{Chunk, Wave};

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
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk {
        let mut i = 0;

        self.0
            .synthesize(elapsed, interval, vars)
            .for_each_sample(|sample| {
                let hz_32 = (*sample * 32.0) as u64;
                let phase = 32_000_000_000 / hz_32;
                let offset = elapsed % phase;
                let place = i * interval / 32 + offset;

                *sample = (place as f32 / phase as f32) % 1.0;
                i += 1;
            })
            .gain(-2.0)
            .offset(1.0)
    }
}
