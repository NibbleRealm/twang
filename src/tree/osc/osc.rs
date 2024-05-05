use crate::tree::{Wave, Chunk};

/// Fixed frequency oscillator (sawtooth wave)
///
/// This is the most basic oscillator, which all other oscillators depend on for
/// their phase.
#[derive(Debug)]
pub struct Osc(pub f32);

impl Wave for Osc {
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
