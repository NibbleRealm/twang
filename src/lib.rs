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
//! - Any sample rate
//! - Bit depth: [8]- or [16]-bit integer and [32]- or [64]-bit float
//! - [Mono], [Stereo], [5.1 Surround] and [7.1 Surround]
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
//!
//! [8]: chan/struct.Ch8.html
//! [16]: chan/struct.Ch16.html
//! [32]: chan/struct.Ch32.html
//! [64]: chan/struct.Ch64.html
//! [Mono]: config/struct.Mono.html
//! [Stereo]: config/struct.Stereo.html
//! [5.1 Surround]: config/struct.Surround.html
//! [7.1 Surround]: config/struct.Surround8.html

#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg",
    html_root_url = "https://docs.rs/twang"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod audio;
pub mod chan;
pub mod gen;
pub mod ops;
pub mod config;
mod private;
pub mod sample;
mod sound;

pub use audio::{Audio, Hz};
