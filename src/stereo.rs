//! Stereo speaker configuration and types.

use crate::{
    chan::{Ch16, Ch32, Ch64, Ch8},
    sample::Sample2,
    Config,
};

/// 2 speaker/channel arrangement (left, right)
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Stereo;

impl Config for Stereo {
    const CHANNEL_COUNT: usize = 2;
}

/// [Stereo](struct.Stereo.html) [8-bit PCM](../chan/struct.Ch8.html) format.
pub type Stereo8 = Sample2<Ch8, Stereo>;
/// [Stereo](struct.Stereo.html) [16-bit PCM](../chan/struct.Ch16.html) format.
pub type Stereo16 = Sample2<Ch16, Stereo>;
/// [Stereo](struct.Stereo.html)
/// [32-bit Floating Point](../chan/struct.Ch32.html) format.
pub type Stereo32 = Sample2<Ch32, Stereo>;
/// [Stereo](struct.Stereo.html)
/// [64-bit Floating Point](../chan/struct.Ch64.html) format.
pub type Stereo64 = Sample2<Ch64, Stereo>;
