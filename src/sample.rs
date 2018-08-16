// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use std::ops::{Neg, Add, AddAssign, Div, DivAssign, Mul, MulAssign};

/// A Sample and Timestamp.
#[derive(Copy, Clone)]
pub struct Sample {
	#[allow(unused)]
	pub(crate) t: f64,
	pub(crate) v: f64,
}

impl Sample {
	/// Mix sound waves together.  (Add soundwaves together, then divide)
	#[inline(always)] pub fn mix(mut self, a: &[Self]) -> Self {
		for i in a.iter() {
			self += *i;
		}

		self /= a.len() + 1;
		self
	}

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

	/// Only the positve part of the wave.
	#[inline(always)] pub fn pos(mut self) -> Self {
		self.v = if self.v > 0.0 { self.v } else { 0.0 };
		self
	}

	/// Only the negative part of the wave.
	#[inline(always)] pub fn neg(mut self) -> Self {
		self.v = if self.v < 0.0 { self.v } else { 0.0 };
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

	/// Discrete step conversion of sound wave.
	#[inline(always)] pub fn dst(mut self, discrete_steps: u16) -> Self {
		/*-1 to 1, 0 to 1, 0 to 255, 0 to 1, 0 to 2, -1 to 1 */
		self.v = (((self.v * 0.5 + 0.5) * discrete_steps as f64).round()
			/ discrete_steps as f64) * 2.0 - 1.0;
		self
	}
}

impl Neg for Sample {
	type Output = Sample;

	/// Invert sound wave (-x).
	fn neg(mut self) -> Sample {
		self.v = -self.v;
		self
	}
}

impl Add for Sample {
	type Output = Sample;

	/// Add sound waves together.  (Add sound waves ontop of eachother), may
	/// introduce clipping.
	fn add(mut self, other: Sample) -> Sample {
		self.v += other.v;
		self
	}
}

impl AddAssign for Sample {
	fn add_assign(&mut self, other: Sample) {
		self.v += other.v;
	}
}

impl Div<usize> for Sample {
	type Output = Sample;

	fn div(mut self, other: usize) -> Sample {
		self.v /= other as f64;
		self
	}
}

impl DivAssign<usize> for Sample {
	fn div_assign(&mut self, other: usize) {
		self.v /= other as f64;
	}
}

impl Mul<f64> for Sample {
	type Output = Sample;

	/// Multiply this sample by an amplitude (non-negative).
	fn mul(mut self, other: f64) -> Sample {
		self.v *= if other < 0.0 { 0.0 } else { other };
		self
	}
}

impl MulAssign<f64> for Sample {
	/// Multiply this sample by an amplitude (non-negative).
	fn mul_assign(&mut self, other: f64) {
		self.v *= if other < 0.0 { 0.0 } else { other };
	}
}

impl Mul for Sample {
	type Output = Sample;

	/// Multiply sound waves together, avoiding octave jump.
	fn mul(mut self, other: Sample) -> Sample {
		// 2 negatives multiplied together should be negative, otherwise
		// the pitch jumps up an octave.
		if self.v < 0.0 && other.v < 0.0 {
			self.v *= -other.v;
		} else {
			self.v *= other.v;
		}
		self
	}
}

impl MulAssign for Sample {
	/// Multiply sound waves together, avoiding octave jump.
	fn mul_assign(&mut self, other: Sample) {
		// 2 negatives multiplied together should be negative, otherwise
		// the pitch jumps up an octave.
		if self.v < 0.0 && other.v < 0.0 {
			self.v *= -other.v;
		} else {
			self.v *= other.v;
		}
	}
}

impl Into<i16> for Sample {
	/// Convert to an i16 sample.
	fn into(self) -> i16 {
		(self.v * (::std::i16::MAX as f64)) as i16
	}
}

/// Trait for mul and mix on slice of samples.
pub trait SampleSlice {
	/// Mix samples together (addition & division).
	fn mix(&self) -> Sample;
	/// Mulitply samples together, avoiding octave jump.
	fn mul(&self) -> Sample;
}

impl SampleSlice for [Sample] {
	fn mix(&self) -> Sample {
		let mut sample = self[0];
		for i in self.iter().skip(1) {
			sample += *i;
		}
		sample /= self.len();
		sample
	}

	fn mul(&self) -> Sample {
		let mut sample = self[0];
		for i in self.iter().skip(1) {
			sample *= *i;
		}
		sample
	}
}
