use crate::config::{Mono, Stereo, Surround, Surround8, Config};
use crate::chan::{Channel, Ch16, Ch32, Ch64, Ch8};
use crate::sample::{Sample1, Sample2, Sample6, Sample8};
use core::any::Any;

pub trait Sealed: Any {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for Ch64 {}

impl Sealed for Mono {}

impl Sealed for Stereo {}

impl Sealed for Surround {}

impl Sealed for Surround8 {}

impl<C: Channel, F: Config> Sealed for Sample1<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample2<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample6<C, F> {}

impl<C: Channel, F: Config> Sealed for Sample8<C, F> {}
