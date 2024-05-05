//! Basic oscillators (wave generators)

const_postfix_waveform!(Osc);
const_postfix_waveform!(Sine<T>, T);

mod sine;
mod osc;

pub use self::{sine::Sine, osc::Osc};
