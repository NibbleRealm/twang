use crate::tree::{Chunk, Wave};

/// Parameterized signal
#[derive(Copy, Clone, Debug)]
pub struct Param(usize);

impl Wave for Param {
    fn synthesize(&self, _elapsed: u64, _interval: u64, vars: &[f32]) -> Chunk {
        Chunk([vars[self.0]; 32])
    }
}
