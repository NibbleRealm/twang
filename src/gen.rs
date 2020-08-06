// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Audio generators

use core::time::Duration;

mod pink;
mod saw;
mod white;

pub use pink::Pink;
pub use saw::Saw;
pub use white::White;

/// A generator for audio.
pub trait Generator {
    /// Sample audio with duration since last sampled.
    fn sample(&mut self, duration: Duration) -> f64;
}
