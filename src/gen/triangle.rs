use crate::Hz;

/// A simple triangle wave generator.
pub struct Triangle {
    value: f64,
}

impl Triangle {
    /// Create a triangle wave generator.
    pub fn new(hertz: Hz) -> Self {
        Self {
            value: 0.0,
        }
    }
}
