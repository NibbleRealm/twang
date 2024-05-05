//! Line signals

const_postfix_waveform!(Line);

use crate::tree::{Wave, Chunk};

/// Constant signal
#[derive(Debug)]
pub struct Line(pub f32);

impl Wave for Line {
    fn synthesize(&self, _elapsed: u64, _interval: u64) -> Chunk {
        Chunk([self.0; 32])
    }
}
