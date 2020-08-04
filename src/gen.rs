use std::time::Duration;

mod pink;
mod white;
mod triangle;

pub use pink::Pink;
pub use white::White;
pub use triangle::Triangle;

/// A generator for audio.
pub trait Generator {
    /// Sample audio with duration since last sampled.
    fn sample(&mut self, duration: Duration) -> f64;
}
