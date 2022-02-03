// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use fon::chan::{Ch32, Channel};

/// Noise gate.
///
/// - `input`: The signal the gate is being applied to.
/// - `key`: The signal that is triggering the gate (often same as input).
/// - `range`: How much the signal below the noise threshold is attenuated.
/// - `open_threshold`: The level at which the gate opens.
/// - `close_threshold`: The level at which the gate closes.
/// - `attack`: How long it takes for gate to fully open (seconds).
/// - `hold`: How long the signal is held before release once below threshold
///   (seconds).
/// - `release`: How long it takes for gate to fully close (seconds).
#[derive(Debug, Clone, Copy, Default)]
pub struct Gate {
    /// The level of the gate (1.0 is fully closed, 0.0 is fully open).
    level: f32,
    /// Non-Infinity if gate is closing.
    hold: f32,
}

impl Gate {
    /// Create a new gate.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get next sample processed through the noise gate.
    #[inline(always)]
    pub fn step(&mut self, gate: &GateParams) -> Ch32 {
        // If gate should open.
        if gate.key.to_f32() >= gate.open_threshold.to_f32() {
            // Adjust level based on attack parameter.
            self.level = if gate.attack == 0.0 {
                0.0
            } else {
                (self.level - (1.0 / 48_000.0) / gate.attack).max(0.0)
            };
            // Reset hold value now that open.
            self.hold = f32::INFINITY;
        }
        // If gate should close.
        if gate.key.to_f32() < gate.close_threshold.to_f32() {
            // If gate state changes to closing, set hold time.
            if self.hold == f32::INFINITY {
                self.hold = gate.hold;
            }
            // If hold is over, start increasing attenuation level.
            if self.hold == 0.0 {
                // Adjust level based on release parameter.
                self.level = if gate.release == 0.0 {
                    1.0
                } else {
                    (self.level + (1.0 / 48_000.0) / gate.release).min(1.0)
                };
            }
        }
        // Adjust hold time.
        if self.hold != f32::INFINITY {
            self.hold = (self.hold - (1.0 / 48_000.0)).max(0.0);
        }
        // Calculate attenuation level.
        let level = 1.0 - gate.range.to_f32() * self.level;
        // Attenuate noise level of signal.
        Ch32::from(level * gate.input.to_f32())
    }
}

/// Parameters of the Noise Gate.
#[derive(Debug, Copy, Clone)]
pub struct GateParams {
    /// The signal the gate is being applied to.
    pub input: Ch32,
    /// The signal that is triggering the gate (often same as input).
    pub key: Ch32,
    /// How much the signal below the noise threshold is attenuated (reduced).
    ///
    /// Set to 1 for silence below threshold, 0 makes the gate have no effect.
    pub range: Ch32,
    /// The level at which the gate opens.
    pub open_threshold: Ch32,
    /// The level at which the gate closes.
    pub close_threshold: Ch32,
    /// How long it takes for gate to fully open (seconds).
    pub attack: f32,
    /// How long the signal is held before release once below threshold
    /// (seconds).
    pub hold: f32,
    /// How long it takes for gate to fully close (seconds).
    pub release: f32,
}
