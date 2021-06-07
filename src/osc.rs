// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! A collection of basic oscillators (wave generators).

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
