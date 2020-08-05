// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Library for audio conversions and synthesis.
//!
//! An [audio buffer](struct.Audio.html) can be cheaply converted to and from
//! raw byte buffers, enabling interoperability with other crates.
//!
//! Many audio formats are supported:
//! - Bit depth: 8- or 16-bit integer and 32- or 64-bit float
//! - All sample rates
//! - Mono, Stereo, 5.1 Surround and 7.1 Surround
//!
//! Synthesis with blending [operations](ops/index.html) is supported for all
//! formats.
//!
//! # A4 (440 Hz) Organ Example
//! ```rust,no_run
//! use cala::speaker::Player;
//! use twang::Sound;
//!
//! let mut speaker = Speaker::new(0, false).unwrap();
//! let mut snds = Sound::new(None, 440.0); // A4
//!
//! loop {
//!     speaker.update(&mut || {
//!         let x = snds.next().unwrap();
//!
//!         (x.sin().positive() + x.tri().negative()).into()
//!     });
//! }
//! ```

mod audio;
pub mod chan;
pub mod gen;
pub mod ops;
mod private;
pub mod sample;
mod sound;

pub use audio::{Audio, Hz};
