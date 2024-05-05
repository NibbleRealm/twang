use fon::chan::Ch32;

/// Signal inverter.
#[derive(Debug, Clone, Copy, Default)]
pub struct Invert;

impl Invert {
    /// Get next inverted sample.
    #[inline(always)]
    pub fn step(&mut self, input: Ch32) -> Ch32 {
        -input
    }
}
