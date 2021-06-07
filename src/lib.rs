// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Library for pure Rust advanced audio synthesis.
//!
//! Most audio DSP (Digital Signal Processing) libraries have a concept of an
//! audio graph which connects sources to destinations.  Twang uses a simplified
//! model: a synthesis tree.  Twang doesn't deal with having speakers as a node
//! on a graph, as it's only focus is synthesis.  A synthesis tree can do all of
//! the things that an audio graph can do, but it's simpler and much easier to
//! learn.
//!
//! To start, first you need to construct a **synthesizer**
//! ([`Synth`](struct.Synth.html)).  Then you need a type that implements the
//! `Sink` trait.  `Audio` buffers have a `sink` method you can use to get a
//! `Sink`.  Once you have those, you can synthesize audio with a closure that
//! has one parameter representing the **frequency counter**.  You can use the
//! **frequency counter** to generate continuous pitched waveforms.
//!
//! # A3 (220 Hz) Minor Piano Example
//! This example uses the first ten piano harmonics to generate a sound that
//! sounds like an electric piano.  This is an example of additive synthesis.
//!
//! ```rust,no_run
//! use fon::chan::Ch16;
//! use fon::{Audio, Frame, Stream};
//! use twang::noise::White;
//! use twang::ops::Gain;
//! use twang::osc::Sine;
//! use twang::Synth;
//!
//! /// First ten harmonic volumes of a piano sample (sounds like electric piano).
//! const HARMONICS: [f32; 10] = [
//!     0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
//! ];
//! /// The three pitches in a perfectly tuned A3 minor chord
//! const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
//! /// Volume of the piano
//! const VOLUME: f32 = 1.0 / 3.0;
//!
//! // State of the synthesizer.
//! #[derive(Default)]
//! struct Processors {
//!     // White noise generator.
//!     white: White,
//!     // 10 harmonics for 3 pitches.
//!     piano: [[Sine; 10]; 3],
//! }
//! 
//! fn main() {
//!     // Initialize audio
//!     let mut audio = Audio::<Ch16, 2>::new(48_000);
//!     // Create audio processors
//!     let mut proc = Processors::default();
//!     // Adjust phases of harmonics.
//!     for pitch in proc.piano.iter_mut() {
//!         for harmonic in pitch.iter_mut() {
//!             harmonic.shift(proc.white.next());
//!         }
//!     }
//!     // Build synthesis algorithm
//!     let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 2>| {
//!         for (s, pitch) in proc.piano.iter_mut().zip(PITCHES.iter()) {
//!             for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
//!                 // Get next sample from oscillator.
//!                 let sample = o.next(pitch * (i + 1) as f32);
//!                 // Pan the generated harmonic center
//!                 frame = frame.pan(Gain.next(sample, (v * VOLUME).into()), 0.0);
//!             }
//!         }
//!         frame
//!     });
//!     // Synthesize 5 seconds of audio
//!     synth.extend(&mut audio, 48_000 * 5);
//! }
//! ```

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

mod synth;

pub mod noise;
pub mod ops;
pub mod osc;

pub use synth::Synth;
