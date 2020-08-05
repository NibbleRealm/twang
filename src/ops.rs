//! Synthesis and mixing operations.
//!
//! Used in `Audio` methods `blend_sample` and `blend_audio`.

use crate::chan::Channel;
use core::any::Any;

/// Blending operation for mixing
pub trait Blend: Any + Copy + Clone {
    /// Synthesize to destination by blending destination and source.
    fn synthesize<C: Channel>(dst: &mut C, src: &C);
}

/// Source only (ignore destination)
#[derive(Clone, Copy, Debug)]
pub struct Src;
/// Destination only (ignore source)
#[derive(Clone, Copy, Debug)]
pub struct Dest;
/// Source or destination with no overlap
#[derive(Clone, Copy, Debug)]
pub struct Xor;
/// Clear (set to default)
#[derive(Clone, Copy, Debug)]
pub struct Clear;
/// VCA (Voltage Controlled Amplitude) mixing.  Multiplication of signals.
#[derive(Clone, Copy, Debug)]
pub struct Gain;
/// Standard audio mixing.  Addition of signals
#[derive(Clone, Copy, Debug)]
pub struct Add;
/// Squared compression audio mixing.  Addition of signals squared.
#[derive(Clone, Copy, Debug)]
pub struct AddSquared;
/// Minimum of destination and source
#[derive(Clone, Copy, Debug)]
pub struct Min;
/// Maximum of destination and source
#[derive(Clone, Copy, Debug)]
pub struct Max;
/// Squared compression.
#[derive(Clone, Copy, Debug)]
pub struct Squared;
/// Apply sine function to input.
#[derive(Clone, Copy, Debug)]
pub struct Sine;
/// Apply sawtooth function to input.
#[derive(Clone, Copy, Debug)]
pub struct Sawtooth;
/// Apply square function to input.
#[derive(Clone, Copy, Debug)]
pub struct Square;

impl Blend for Src {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src;
    }
}

impl Blend for Dest {
    fn synthesize<C: Channel>(_dst: &mut C, _src: &C) {
        // leave _dst as is
    }
}

impl Blend for Xor {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = if *dst == C::MID {
            *src
        } else if *src == C::MID {
            *dst
        } else {
            C::MID
        };
    }
}

impl Blend for Clear {
    fn synthesize<C: Channel>(dst: &mut C, _src: &C) {
        *dst = C::default();
    }
}

impl Blend for Gain {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src * *dst;
    }
}

impl Blend for Add {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = *src + *dst;
    }
}

impl Blend for AddSquared {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        Add::synthesize(dst, src);
        Square::synthesize(dst, src);
    }
}

impl Blend for Min {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = (*dst).min(*src);
    }
}

impl Blend for Max {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = (*dst).max(*src);
    }
}

impl Blend for Squared {
    fn synthesize<C: Channel>(dst: &mut C, _src: &C) {
        *dst = *dst * *dst;
    }
}

impl Blend for Sine {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = C::from((dst.to_f64() * std::f64::consts::PI).sin()) * *src;
    }
}

impl Blend for Sawtooth {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = C::from(dst.to_f64().abs() * 2.0 - 1.0) * *src;
    }
}

impl Blend for Square {
    fn synthesize<C: Channel>(dst: &mut C, src: &C) {
        *dst = C::from(dst.to_f64().signum()) * *src;
    }
}
