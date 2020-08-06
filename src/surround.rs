//! Surround speaker configurations and types.

use crate::{
    chan::{Ch16, Ch32, Ch64, Ch8},
    sample::{Sample6, Sample8},
    Config,
};

/// 6 speaker/channel arrangement (ITU 5.1 Surround Sound Standard)
/// - front left
/// - front right
/// - front center
/// - back left
/// - back right
/// - lfe (low frequency effects)
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Surround;

impl Config for Surround {
    const CHANNEL_COUNT: usize = 6;
}

/// [5.1 Surround](struct.Surround.html) [8-bit PCM](../chan/struct.Ch8.html)
/// format.
pub type Surround8 = Sample6<Ch8, Surround>;
/// [5.1 Surround](struct.Surround.html) [16-bit PCM](../chan/struct.Ch16.html)
/// format.
pub type Surround16 = Sample6<Ch16, Surround>;
/// [5.1 Surround](struct.Surround.html)
/// [32-bit Floating Point](../chan/struct.Ch32.html) format.
pub type Surround32 = Sample6<Ch32, Surround>;
/// [5.1 Surround](struct.Surround.html)
/// [64-bit Floating Point](../chan/struct.Ch64.html) format.
pub type Surround64 = Sample6<Ch64, Surround>;

/// 8 speaker/channel arrangement (Blu-ray / Dolby 7.1 Surround Sound Standard)
/// - front left
/// - front right
/// - front center
/// - back left
/// - back right
/// - lfe (low frequency effects)
/// - side left
/// - side right
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct SurroundHD;

impl Config for SurroundHD {
    const CHANNEL_COUNT: usize = 8;
}

/// [7.1 Surround](struct.SurroundHD.html) [8-bit PCM](../chan/struct.Ch8.html)
/// format.
pub type SurroundHD8 = Sample8<Ch8, SurroundHD>;
/// [7.1 Surround](struct.SurroundHD.html)
/// [16-bit PCM](../chan/struct.Ch16.html) format.
pub type SurroundHD16 = Sample8<Ch16, SurroundHD>;
/// [7.1 Surround](struct.SurroundHD.html)
/// [32-bit Floating Point](../chan/struct.Ch32.html) format.
pub type SurroundHD32 = Sample8<Ch32, SurroundHD>;
/// [7.1 Surround](struct.SurroundHD.html)
/// [64-bit Floating Point](../chan/struct.Ch64.html) format.
pub type SurroundHD64 = Sample8<Ch64, SurroundHD>;
