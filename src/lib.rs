// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Library for pure Rust advanced audio synthesis.
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
//! use twang::gen::{Generator, Saw};
//! use fon::{
//!     mono::Mono64,
//!     ops::{Add, Sine},
//!     Audio, Hz,
//! };
//!
//! /// First ten harmonic volumes of a piano sample (sounds like electric piano).
//! const HARMONICS: [f64; 10] = [
//!     0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
//! ];
//! /// The three pitches in a perfectly tuned A3 minor chord
//! const PITCHES: [f64; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
//!
//! let mut gen;
//!
//! // Five seconds of 48 KHz Audio
//! let mut chord = Audio::with_silence(48_000, 48_000 * 5);
//! let mut temp;
//!
//! // Synthesize an A minor chord.
//! let volume = 0.25; // To avoid clipping
//! for pitch in PITCHES.iter().cloned() {
//!     // Add note to chord
//!     for (i, harmonic) in HARMONICS.iter().cloned().enumerate() {
//!         let i: f64 = (i as i32).into();
//!         gen = Saw::new(Hz(pitch * i));
//!         temp = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
//!         gen.generate(&mut temp);
//!         temp.blend_sample(Mono64::new(harmonic * volume), Sine);
//!         // Add harmonic to chord
//!         chord.blend_audio(&temp, Add);
//!     }
//! }
//! ```
//!
//! [8]: chan/struct.Ch8.html
//! [16]: chan/struct.Ch16.html
//! [32]: chan/struct.Ch32.html
//! [64]: chan/struct.Ch64.html
//! [Mono]: mono/struct.Mono.html
//! [Stereo]: stereo/struct.Stereo.html
//! [5.1 Surround]: surround/struct.Surround.html
//! [7.1 Surround]: surround/struct.SurroundHD.html

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

pub mod gen;
