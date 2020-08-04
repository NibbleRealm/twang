// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! A sound synthesis crate.
//!
//! # A4 (440 Hz) Organ Example
//! ```rust,no_run
//! use cala::speaker::Player;
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
//!             (x.sin().positive() + x.tri().negative()).into()
//!         });
//!     }
//! }

mod audio;
pub mod ops;
pub mod chan;
pub mod gen;
mod private;
pub mod sample;
mod sound;

pub use audio::{Audio, Hz};
