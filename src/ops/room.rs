// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use fon::chan::{Ch32, Channel};
use std::collections::VecDeque;

/// Room effect.  Use to add reflections to the sound.  Reflections can create
/// either echo (> 50 ms delay) or reverb (< 30 ms delay).
#[derive(Debug, Clone, Default)]
pub struct Room {
    buffer: VecDeque<Ch32>,
}

impl Room {
    /// Create a new Room Effect.
    #[inline(always)]
    pub fn new() -> Self {
        Room {
            buffer: VecDeque::new(),
        }
    }

    /// Add reflection to the room.
    /// - `signal`: the input signal
    /// - `seconds`: the number of seconds it takes for the reflection to occur
    /// - `attenuation`: the gain to multiply by after each reflection
    #[inline(always)]
    pub fn add(&mut self, signal: Ch32, seconds: f32, attenuation: f32) {
        let offset = (48_000.0 * seconds) as usize;
        self.buffer.resize(offset + 1, Ch32::default());
        self.buffer[offset] += Ch32::new(signal.to_f32() * attenuation);
    }

    /// Generate the next sample of all reflections in the room.
    #[inline(always)]
    pub fn step(&mut self) -> Ch32 {
        self.buffer.pop_front().unwrap_or_default()
    }
}
