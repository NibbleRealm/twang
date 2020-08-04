use crate::chan::{Ch8, Ch16, Ch32, Ch64};
use crate::audio::{Mono, Stereo, Surround, SurroundTheater};
use std::any::Any;

pub trait Sealed: Any {}

impl Sealed for Ch8 {}

impl Sealed for Ch16 {}

impl Sealed for Ch32 {}

impl Sealed for Ch64 {}

impl Sealed for Mono {}

impl Sealed for Stereo {}

impl Sealed for Surround {}

impl Sealed for SurroundTheater {}
