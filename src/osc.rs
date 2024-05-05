//! A collection of basic oscillators (wave generators).

#![allow(warnings)]

mod pulse;
mod sawtooth;
mod sine;
mod triangle;

use core::f32::consts::TAU;

pub use pulse::Pulse;
pub use sawtooth::Sawtooth;
pub use sine::Sine;
pub use triangle::Triangle;

// Seconds per sample.
const SAMPLE_PERIOD: f32 = 1.0 / 48_000.0;
