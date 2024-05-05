use crate::tree::{Chunk, Wave};

/// Constant signal
#[derive(Copy, Clone, Debug)]
pub struct Line(pub f32);

impl Wave for Line {
    fn synthesize(
        &self,
        _elapsed: u64,
        _interval: u64,
        _vars: &[f32],
    ) -> Chunk {
        Chunk([self.0; 32])
    }
}
