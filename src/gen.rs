//! Audio generators

use core::time::Duration;

mod pink;
mod saw;
mod white;

pub use pink::Pink;
pub use saw::Saw;
pub use white::White;

/// A generator for audio.
pub trait Generator {
    /// Sample audio with duration since last sampled.
    fn sample(&mut self, duration: Duration) -> f64;
}
