// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Sample;

/// Quiet sampler (Silence).
pub struct Quiet {
	time: f64,
	step: f64,
}

impl Quiet {
	/// Create a new sampler at sample rate 48KHz or specific Hz.
	#[inline(always)]
	pub fn new(hz: Option<f64>) -> Self {
		let hz = hz.unwrap_or(48_000.0);

		Self { time: 0.0, step: 1.0 / hz }
	}
}

impl Iterator for Quiet {
	type Item = Sample;

	#[inline(always)]
	fn next(&mut self) -> Option<Sample> {
		let sample = Sample {
			t: self.time,
			v: 0.0,
		};

		self.time += self.step;

		Some(sample)
	}
}
