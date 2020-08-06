// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::chan::{Ch16, Ch32, Ch64, Ch8, Channel};
use crate::mono::Mono;
use crate::sample::{Sample1, Sample2, Sample6, Sample8};
use crate::stereo::Stereo;
use crate::surround::{Surround, SurroundHD};
use crate::Config;
use core::any::Any;

pub trait Sealed: Any {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for Ch64 {}

impl Sealed for Mono {}

impl Sealed for Stereo {}

impl Sealed for Surround {}

impl Sealed for SurroundHD {}

impl<C: Channel, F: Config> Sealed for Sample1<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample2<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample6<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample8<C, F> {}
