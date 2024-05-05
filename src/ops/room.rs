use alloc::collections::VecDeque;
use fon::chan::{Ch32, Channel};

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
