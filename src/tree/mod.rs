//! Library for pure Rust advanced audio synthesis.

macro_rules! const_postfix_waveform {
    () => {
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
mod synth;
pub mod osc;
pub mod line;

use self::chunk::Chunk;
pub use self::synth::Synth;

/// Trait implemented by all waveforms
#[traitful::seal(osc::Osc, for<T: Wave> osc::Sine<T>, line::Line, for<T: Wave> &T)]
pub trait Wave {
    /// Synthesize a chunk of audio.
    ///
    /// - `elapsed` is nanoseconds since the start of synthesis (up to about
    ///   1169 years)
    /// - `interval` is nanoseconds in the chunk's interval
    #[must_use]
    #[doc(hidden)]
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk;
}

impl<T> Wave for &T
where
    T: Wave,
{
    fn synthesize(&self, elapsed: u64, interval: u64) -> Chunk {
        (**self).synthesize(elapsed, interval)
    }
}
