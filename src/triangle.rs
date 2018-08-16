// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Quiet;
use Sample;

/// Triangle Sampler.
pub struct Triangle(Quiet, f64);

impl Triangle {
	/// Create a new Triangle Sampler.
	pub fn new(hz: Option<f64>, note: f64) -> Self {
		Triangle(Quiet::new(hz), note)
	}
}

impl Iterator for Triangle {
	type Item = Sample;

	fn next(&mut self) -> Option<Sample> {
		let mut sample = self.0.next().unwrap();
		let x = sample.t * self.1;
		sample.v = ((x % 1.0) * 2.0 - 1.0).abs() * 2.0 - 1.0;
		Some(sample)
	}
}
