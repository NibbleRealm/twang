// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Speaker/channel configuration.

use crate::private::Sealed;
use std::fmt::Debug;

/// Speaker/Channel configuration
pub trait Config: Copy + Clone + Debug + Default + PartialEq + Sealed {
    /// Number of channels for this configuration
    const CHANNEL_COUNT: usize;
}
