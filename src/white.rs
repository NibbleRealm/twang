// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use quiet::Quiet;
use Sample;

use rand::{thread_rng, ThreadRng, distributions::{Uniform, Distribution}};

/// White Noise Sampler.
pub struct White {
	dist: Uniform<f64>,
	rng: ThreadRng,
	sampler: Quiet,
}

impl White {
	/// Create a new White Noise Sampler.
	pub fn new(hz: Option<f64>) -> Self {
		Self {
			dist: Uniform::new_inclusive(-1.0, 1.0),
			rng: thread_rng(),
			sampler: Quiet::new(hz),
		}
	}

	fn sample(&mut self) -> f64 {
		self.dist.sample(&mut self.rng)
	}
}

impl Iterator for White {
	type Item = Sample;

	fn next(&mut self) -> Option<Sample> {
		let mut sample = self.sampler.next().unwrap();
		sample.v = self.sample();
		Some(sample)
	}
}
