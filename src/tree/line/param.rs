use crate::tree::{Chunk, Data, Wave};

/// Parameterized signal
#[derive(Copy, Clone, Debug)]
pub struct Param(usize);

impl Wave for Param {
    const STATE_LEN: usize = 0;

    fn synthesize(&self, data: &mut Data<'_>) -> Chunk {
        data.params.chunk(self.0)
    }
}
