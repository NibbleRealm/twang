use core::f32::consts;

use crate::tree::{Chunk, Wave};

/// Sine wave
///
/// Takes phase (-1 to 1) as input
#[derive(Debug)]
pub struct Sine<I>(pub I);

impl<I> Wave for Sine<I>
where
    I: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk {
        self.0
            .synthesize(elapsed, interval, vars)
            .gain(consts::PI)
            .cosine()
            .invert()
    }
}
