use std::time::Duration;

mod pink;
mod triangle;
mod white;

pub use pink::Pink;
pub use triangle::Triangle;
pub use white::White;

/// A generator for audio.
pub trait Generator {
    /// Sample audio with duration since last sampled.
    fn sample(&mut self, duration: Duration) -> f64;
}
