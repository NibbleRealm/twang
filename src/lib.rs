// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! A sound synthesis crate.
//!
//! # A4 (440 Hz) Organ Example
//! ```rust
//! extern crate twang; // for sound generation / effects
//! extern crate adi; // for speaker
//!
//! use adi::speaker::Speaker;
//! use twang::Sound;
//!
//! fn main() {
//!     let mut speaker = Speaker::new(0, false).unwrap();
//!     let mut snds = Sound::new(None, 440.0); // A4
//!
//!     loop {
//!         speaker.update(&mut || {
//!             let x = snds.next().unwrap();
//!
//!             (x.sin().pos() + x.tri().neg()).into()
//!         });
//!     }
//! }

extern crate rand; // for noise generation

mod pink;
mod white;
mod sample;
mod quiet;
mod sound;

pub use crate::pink::Pink;
pub use crate::white::White;
pub use crate::sample::Sample;
pub use crate::sound::{Sound,Wave};

/// Traits
pub mod prelude {
    pub use crate::sample::SampleSlice;
}
