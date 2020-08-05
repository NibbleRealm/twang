//! Speaker/channel configuration.

use crate::private::Sealed;
use std::fmt::Debug;

/// 1 Channel (front center)
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Mono;
/// 2 Channels (front left, front right)
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Stereo;
/// 6 Channels ITU 5.1 Surround Sound Standard (most common surround sound
/// configuration).
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Surround;
/// 8 Channels Blu-ray / Dolby 7.1 Surround Sound.
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Surround8;

/// Speaker/Channel configuration
pub trait Config: Copy + Clone + Debug + Default + PartialEq + Sealed {
    /// Number of channels for this configuration
    const CHANNEL_COUNT: usize;
}

impl Config for Mono {
    const CHANNEL_COUNT: usize = 1;
}

impl Config for Stereo {
    const CHANNEL_COUNT: usize = 2;
}

impl Config for Surround {
    const CHANNEL_COUNT: usize = 6;
}

impl Config for Surround8 {
    const CHANNEL_COUNT: usize = 8;
}
