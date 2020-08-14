// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::sig::Signal;

/// Trait for synthesizing multiple sounds together.
pub trait Synth {
    /// Add two signals together.
    fn mix(self) -> Signal;
}

impl<I: IntoIterator<Item = Signal>> Synth for I {
    fn mix(self) -> Signal {
        self.into_iter().map(f64::from).sum::<f64>().into()
    }
}
