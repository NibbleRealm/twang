use crate::tree::{Chunk, Data, Wave};

/// Constant signal
#[derive(Copy, Clone, Debug)]
pub struct Line(pub f32);

impl Wave for Line {
    const STATE_LEN: usize = 0;

    fn synthesize(&self, _data: &mut Data<'_>) -> Chunk {
        Chunk([self.0; 32])
    }
}
