//! Speaker/channel configuration.

use crate::private::Sealed;
use std::fmt::Debug;

/// Speaker/Channel configuration
pub trait Config: Copy + Clone + Debug + Default + PartialEq + Sealed {
    /// Number of channels for this configuration
    const CHANNEL_COUNT: usize;
}
