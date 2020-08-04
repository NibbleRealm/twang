use super::Generator;
use std::time::Duration;

use crate::Hz;

/// A simple triangle wave generator.
pub struct Triangle {
    hertz: Hz,
    value: f64,
}

impl Triangle {
    /// Create a triangle wave generator.
    pub fn new(hertz: Hz) -> Self {
        let value = -1.0;
        Self { hertz, value }
    }
}

impl Generator for Triangle {
    fn sample(&mut self, duration: Duration) -> f64 {
        self.value = (self.value + duration.as_secs_f64() * self.hertz.0) % 1.0;
        self.value * 2.0 - 1.0
    }
}
