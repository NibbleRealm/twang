// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Sample types

use crate::Hz;
use crate::chan::Channel;
use crate::ops::Blend;
use crate::private::Sealed;
use std::ops::{Neg, Add, AddAssign, Div, DivAssign, Mul, MulAssign};
use std::fmt::Debug;

/*
/// Distort sound wave with hard clipping.  Volume should be more than 1 to have
/// any effect.
#[inline(always)] pub fn hrd(mut self, volume: f64) -> Self {
    self.v = (self.v * volume).min(1.0).max(-1.0);
    self
}

/// Distort sound wave with soft clipping.  Volume should be more than 1 to have
/// any effect.
#[inline(always)] pub fn sft(mut self, volume: f64) -> Self {
    let max = (1.0 / (1.0 + (-volume).exp()) ) * 2.0 - 1.0;

    self.v = ((1.0 / (1.0 + (self.v * -volume).exp()) ) * 2.0 - 1.0) / max;
    self
}

/// X power of sound wave.  x=2 for squaring, x=1/2 for square root.
#[inline(always)] pub fn pow(mut self, x: f64) -> Self {
    self.v = self.v.powf(x);
    self
}

/// Signum of sound wave (-1 or 1)
#[inline(always)] pub fn sgn(mut self) -> Self {
    self.v = self.v.signum();
    self
}
*/

pub trait Sample: Clone + Copy + Debug + Default + PartialEq + Sealed {
    /// Channel type
    type Chan: Channel;
    /// Sample rate (in Hertz)
    const RATE: Hz;
    /// Number of channels
    const LEN: usize;

    /// Get the channels.
    fn channels(&self) -> &[Self::Chan];

    /// Get the channels mutably.
    fn channels_mut(&mut self) -> &mut [Self::Chan];

    /// Make a pixel from a slice of channels.
    fn from_channels(ch: &[Self::Chan]) -> Self;

    /// Synthesis of a sample with a slice of samples.
    fn blend_sample<O>(dst: &mut [Self], sample: &Self, op: O)
    where
        O: Blend,
    {
        for d in dst.iter_mut() {
            d.blend_channels(sample, op);
        }
    }

    /// Synthesis of two slices of samples.
    fn blend_slice<O>(dst: &mut [Self], src: &[Self], op: O)
    where
        O: Blend,
    {
        for (d, s) in dst.iter_mut().zip(src) {
            d.blend_channels(s, op);
        }
    }

    /// Synthesize two sample slices together.
    fn blend_channels<O>(&mut self, src: &Self, op: O)
        where O: Blend
    {
        for (d, s) in self.channels_mut().iter_mut().zip(src.channels().iter()) {
            O::synthesize(d, s)
        }
    }
    
    #[inline(always)]
    fn convert<D>(self) -> D
    where
        D: Sample,
        D::Chan: From<Self::Chan> + From<f64>,
    {
        // Convert channels
        match (Self::LEN, D::LEN) {
            // 1:1 sampling (no resample)
            (a, b) if a == b => {
                let mut chans = [D::Chan::MID; 8];
                for (d, s) in chans.iter_mut().zip(self.channels().iter()) {
                    *d = (*s).into();
                }
                D::from_channels(&chans[..self.channels().len()])
            }
            // Downsampling
            (2, 1) => {
                let mut sum = 0.0;
                for chan in self.channels() {
                    sum += chan.to_f64() * 0.5;
                }
                D::from_channels(&[D::Chan::from(sum)])
            }
            (6, 1) => {
                let mut sum = 0.0;
                for chan in self.channels()[0..5].iter() {
                    sum += chan.to_f64() * 0.2;
                }
                sum += self.channels()[5].to_f64(); // LFE
                D::from_channels(&[D::Chan::from(sum)])
            }
            (8, 1) => {
                let mut sum = 0.0;
                for chan in self.channels()[0..7].iter() {
                    sum += chan.to_f64() * (1.0 / 7.0);
                }
                sum += self.channels()[7].to_f64(); // LFE
                D::from_channels(&[D::Chan::from(sum)])
            }
            (6, 2) => {
                let mut left = self.channels()[0].to_f64() * (1.0 / 3.0);
                let mut right = self.channels()[1].to_f64() * (1.0 / 3.0);
                left += self.channels()[2].to_f64() * (1.0 / 3.0);
                right += self.channels()[2].to_f64() * (1.0 / 3.0);
                left += self.channels()[3].to_f64() * (1.0 / 3.0);
                right += self.channels()[4].to_f64() * (1.0 / 3.0);
                left += self.channels()[5].to_f64(); // left LFE
                right += self.channels()[5].to_f64(); // right LFE
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            (8, 2) => {
                let mut left = self.channels()[0].to_f64() * 0.25;
                let mut right = self.channels()[1].to_f64() * 0.25;
                left += self.channels()[2].to_f64() * 0.25;
                right += self.channels()[2].to_f64() * 0.25;
                left += self.channels()[3].to_f64() * 0.25;
                right += self.channels()[4].to_f64() * 0.25;
                left += self.channels()[5].to_f64(); // left LFE
                right += self.channels()[5].to_f64(); // right LFE
                left += self.channels()[6].to_f64() * 0.25;
                right += self.channels()[7].to_f64() * 0.25;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            (8, 5) => {
                let mut left = self.channels()[0].to_f64() * (2.0 / 3.0);
                let mut right = self.channels()[1].to_f64() * (2.0 / 3.0);
                let mut center = self.channels()[2].to_f64();
                let mut back_left = self.channels()[3].to_f64() * (2.0 / 3.0);
                let mut back_right = self.channels()[4].to_f64() * (2.0 / 3.0);
                let mut lfe = self.channels()[5].to_f64();
                left += self.channels()[6].to_f64() * (1.0 / 3.0);
                right += self.channels()[7].to_f64() * (1.0 / 3.0);
                back_left += self.channels()[6].to_f64() * (1.0 / 3.0);
                back_right += self.channels()[7].to_f64() * (1.0 / 3.0);
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right)])
            }
            // Upsampling
            (1, 2) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono)])
            }
            (1, 6) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono)])
            }
            (1, 8) => {
                let mono = self.channels()[0];
                D::from_channels(&[D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono), D::Chan::from(mono)])
            }
            (2, 6) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = left * 0.5 + right * 0.5;
                let lfe = D::Chan::MID;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe)])
            }
            (2, 8) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = left * 0.5 + right * 0.5;
                let lfe = D::Chan::MID;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe), D::Chan::from(left), D::Chan::from(right)])
            },
            (5, 8) => {
                let left = self.channels()[0].to_f64();
                let right = self.channels()[1].to_f64();
                let center = self.channels()[2].to_f64();
                let back_left = self.channels()[3].to_f64();
                let back_right = self.channels()[4].to_f64();
                let lfe = self.channels()[5].to_f64();
                let side_left = (left + back_left) * 0.5;
                let side_right = (right + back_right) * 0.5;
                D::from_channels(&[D::Chan::from(left), D::Chan::from(right), D::Chan::from(center), D::Chan::from(left), D::Chan::from(right), D::Chan::from(lfe), D::Chan::from(side_left), D::Chan::from(side_right)])
            },
            // Unreachable because of sealed traits
            (_, _) => unreachable!(),
        }
    }
}
