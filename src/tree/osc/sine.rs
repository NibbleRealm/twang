use core::f32::consts;

use crate::tree::{Wave, Chunk};

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
