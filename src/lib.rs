// Copyright Jeron Aldaron Lau 2018 - 2020.
// Distributed under either the Apache License, Version 2.0
//    (See accompanying file LICENSE_APACHE_2_0.txt or copy at
//          https://apache.org/licenses/LICENSE-2.0),
// or the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_BOOST_1_0.txt or copy at
//          https://www.boost.org/LICENSE_1_0.txt)
// at your option. This file may not be copied, modified, or distributed except
// according to those terms.

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
//! sounds like an electric piano.  This is an example of additive synthesis,
//! since it uses the `Mix` trait.
//!
//! ```rust,no_run
//! use fon::{mono::Mono64, Audio};
//! use twang::{Synth, Mix};
//!
//! // Target sample rate set to 48 KHz
//! const S_RATE: u32 = 48_000;
//!
//! /// First ten harmonic volumes of a piano sample.
//! const HARMONICS: [f64; 10] = [
//!     0.700,
//!     0.243,
//!     0.229,
//!     0.095,
//!     0.139,
//!     0.087,
//!     0.288,
//!     0.199,
//!     0.124,
//!     0.090,
//! ];
//! /// The three pitches in a perfectly tuned A3 minor chord
//! const PITCHES: [f64; 3] = [
//!     220.0,
//!     220.0 * 32.0 / 27.0,
//!     220.0 * 3.0 / 2.0
//! ];
//!
//! /// Volume of the piano
//! const VOLUME: f64 = 0.1;
//!
//! // Initialize audio with five seconds of silence.
//! let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
//! // Create the synthesizer.
//! let mut synth = Synth::new();
//! // Generate audio samples.
//! synth.gen(audio.sink(..), |fc| {
//!     // Tree-style synthesis
//!     PITCHES
//!        .iter()
//!        .map(|p| {
//!            HARMONICS
//!                .iter()
//!                .enumerate()
//!                .map(|(i, v)| {
//!                    fc.freq(p * (i + 1) as f64).sine().gain(v * VOLUME)
//!                })
//!                .mix()
//!        })
//!        .mix()
//! });
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

mod pink;
mod room;
mod sig;
mod synth;
mod white;

pub use pink::Pink;
pub use room::Room;
pub use sig::Signal;
pub use synth::{Fc, Mix, Synth};
pub use white::White;
