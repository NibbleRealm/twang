// Copyright Jeron Aldaron Lau 2018 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

use crate::{Mix, Signal};

/// Room effect.  Use to add reflections to the sound.  Reflections can create
/// either echo (> 50 ms delay) or reverb (< 50 ms delay).
#[derive(Debug)]
pub struct Room {
    buffer: Vec<Signal>,
    counter: usize,
}

impl Room {
    /// Create a new Room Effect.
    ///
    /// - `queue_len`: Maximum number of samples ahead to generate reverb/echo.
    #[inline(always)]
    pub fn new(queue_len: usize) -> Self {
        Room {
            buffer: vec![0.0.into(); queue_len],
            counter: 0,
        }
    }

    /// Add reflection to the room.
    /// - `signal`: the input signal
    /// - `samples`: the number of samples it takes for the reflection to occur
    /// - `attenuation`: the gain to multiply by after each reflection
    #[inline(always)]
    pub fn add(&mut self, signal: Signal, samples: usize, attenuation: f64) {
        let index = (self.counter + samples) % self.buffer.len();
        self.buffer[index] =
            [self.buffer[index], signal.gain(attenuation)].mix();
    }

    /// Generate the next sample of all reflections in the room.
    #[inline(always)]
    pub fn gen(&mut self) -> Signal {
        let ret = self.buffer[self.counter];
        self.buffer[self.counter] = 0.0.into();
        self.counter = (self.counter + 1) % self.buffer.len();
        ret
    }
}
