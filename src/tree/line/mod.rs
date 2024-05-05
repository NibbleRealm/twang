//! Line signals (silence generators)

#![allow(clippy::module_inception)]

const_postfix_waveform!(Line);
const_postfix_waveform!(Param);

mod line;
mod param;

pub use self::{line::Line, param::Param};
