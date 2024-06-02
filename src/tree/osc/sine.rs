use core::f32::consts;

use crate::tree::{Chunk, Data, Wave};

/// Sine wave
///
/// Takes phase (-1 to 1) as input
#[derive(Debug)]
pub struct Sine<I>(pub I);

impl<I> Wave for Sine<I>
where
    I: Wave,
{
    const STATE_LEN: usize = I::STATE_LEN;

    fn synthesize(&self, data: &mut Data<'_>) -> Chunk {
        self.0.synthesize(data).gain(consts::PI).cosine().invert()
    }
}
