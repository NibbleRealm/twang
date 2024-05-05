//! Basic oscillators (wave generators)

#![allow(clippy::module_inception)]

const_postfix_waveform!(Osc<T>, T);
const_postfix_waveform!(Sine<T>, T);

mod osc;
mod sine;

pub use self::{osc::Osc, sine::Sine};
