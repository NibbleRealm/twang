//! Basic oscillators (wave generators)

#![allow(clippy::module_inception)]

const_postfix_waveform!(Bezier<T, U>, T, U);
const_postfix_waveform!(Osc<T>, T);
const_postfix_waveform!(Pulse<T, U, V>, T, U, V);
const_postfix_waveform!(Sine<T>, T);

mod bezier;
mod osc;
mod pulse;
mod sine;

pub use self::{bezier::Bezier, osc::Osc, pulse::Pulse, sine::Sine};
