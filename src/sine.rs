// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Quiet;
use Sample;



/// Sine Sampler.
pub struct Sine(Quiet, f64);

impl Sine {
	/// Create a new Sine Sampler.
	pub fn new(hz: Option<f64>, note: f64) -> Self {
		Sine(Quiet::new(hz), note)
	}
}

impl Iterator for Sine {
	type Item = Sample;

	fn next(&mut self) -> Option<Sample> {
		let mut sample = self.0.next().unwrap();
		let x = sample.t * self.1;
		sample.v = sin(x);
		Some(sample)
	}
}

/// Harmonic Series Derived Sound Sampler.
pub struct Series<'a>(Quiet, f64, &'a [f64]);

impl<'a> Series<'a> {
	/// Create a new Harmonic Series Derived Sound Sampler.
	pub fn new(hz: Option<f64>, note: f64, overtones: &'a [f64]) -> Self {
		Series(Quiet::new(hz), note, overtones)
	}
}

impl<'a> Iterator for Series<'a> {
	type Item = Sample;

	fn next(&mut self) -> Option<Sample> {
		let mut sample = self.0.next().unwrap();
		let x = sample.t * self.1;
		sample.v = ovr(x, self.2);
		Some(sample)
	}
}
