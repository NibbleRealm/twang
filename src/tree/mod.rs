//! **Library for advanced audio synthesis and mixing**
//!
//! Any synthesized waveform is generated by a dependency tree, also known as a
//! directed acyclic graph (DAG).  The root of the tree is the final sound, with
//! the branches being the intermediate sounds that will eventually make up the
//! result.  This makes synthesized audio work really well with functional
//! programming, since it's just a tree of operations.  A dependency tree for
//! synthesizing audio will be referred to as a "synthesis tree".
//!
//! Twang provides a guarantee that all synthesis trees can be constructed in a
//! `const` context (but don't worry - you can still parameterize your
//! waveforms!).  You can then generate the audio, in chunks, as needed.  Twang
//! hardcodes the chunk size to 32 samples (about 667 microseconds in 48kHz, and
//! 726 microseconds in 44.1kHz).  Through trial and error, it was found that
//! this is the optimal chunk size for realtime audio synthesis.
//!
//! # Getting Started
//!
//! A fairly simple example is making a sine wave:
//!
//! ```rust
//! use fon::{chan::Ch16, Audio};
//! use twang::tree::{line::Line, Synth};
//!
//! // Define waveform
//! let waveform = const { Line(440.0).osc().sine() };
//! // Initialize audio, and create synthesizer
//! let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
//! let mut synth = Synth::new(waveform);
//!
//! // Synthesize 5 seconds of audio
//! synth.stream(audio.sink(), &[]);
//! ```
//!
//! ## Walking Through It
//!
//! 1. When defining the waveform, we start with a line signal at 440.0 (this is
//!    way out of bounds for audio - which is contained between -1.0 and 1.0,
//!    but that's ok since the signal will be changed in the next step).
//! 2. Next, we call `.osc()` which converts that signal into an oscillating
//!    signal at the frequency specified by the previous signal in hertz
//!    (440hz).  This new signal spans the entire range of -1 to 1.
//! 3. And last, we adapt the oscillating signal into a sine wave with `.sine()`
//!
//! Now we need an audio buffer, so we create one with silence (it doesn't
//! matter what we put there, since it will get overwritten anyway)
//!
//! After that, a synthesizer is built with our waveform definition.
//!
//! And finally, the synthesizer streams the generated audio into the sink,
//! overwriting the audio buffer.

macro_rules! const_postfix_waveform {
    () => {
        /// Postfix helper for wrapping synth instruction with [`osc::Osc`].
        ///
        /// [`osc::Osc`]: crate::tree::osc::Osc
        pub const fn osc(self) -> crate::tree::osc::Osc<Self> {
            crate::tree::osc::Osc(self)
        }

        /// Postfix helper for wrapping synth instruction with [`osc::Sine`].
        ///
        /// [`osc::Sine`]: crate::tree::osc::Sine
        pub const fn sine(self) -> crate::tree::osc::Sine<Self> {
            crate::tree::osc::Sine(self)
        }
    };
    ($type:ty) => {
        impl $type {
            const_postfix_waveform!();
        }
    };
    ($type:ty, $($generic:ident),+) => {
       impl<$($generic),+> $type {
           const_postfix_waveform!();
       }
   };
}

mod chunk;
pub mod line;
pub mod osc;
mod synth;

use self::chunk::Chunk;
pub use self::synth::Synth;

/// Trait implemented by all waveforms
#[traitful::seal(
    line::Line,
    line::Param,
    for<T: Wave> &T,
    for<T: Wave> osc::Osc<T>,
    for<T: Wave> osc::Sine<T>,
)]
pub trait Wave {
    /// Synthesize a chunk of audio.
    ///
    /// - `elapsed` is nanoseconds since the start of synthesis (up to about
    ///   1169 years)
    /// - `interval` is nanoseconds in the chunk's interval
    #[must_use]
    #[doc(hidden)]
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk;
}

impl<T> Wave for &T
where
    T: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64, vars: &[f32]) -> Chunk {
        (**self).synthesize(elapsed, interval, vars)
    }
}
