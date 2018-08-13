// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use rand::{thread_rng, ThreadRng, distributions::{Uniform, Distribution}};

/// White Noise Generator.
pub struct WhtGenerator {
	dist: Uniform<f64>,
	rng: ThreadRng,
}

impl WhtGenerator {
	/// Create a new White Noise Generator.
	pub fn new() -> Self {
		Self {
			dist: Uniform::new_inclusive(-1.0, 1.0),
			rng: thread_rng(),
		}
	}

	pub(crate) fn gen(&mut self) -> f64 {
		self.dist.sample(&mut self.rng)
	}
}
