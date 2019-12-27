// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Sample;

/// Pitched Sound sampler.
pub struct Sound {
	time: f64,
	step: f64,
	pitch: f64, // hz
	state: f64, // 0-1
}

impl Sound {
	/// Create a new sampler at sample rate 48KHz or specific Hz.
	#[inline(always)]
	pub fn new(hz: Option<f64>, pitch: f64) -> Self {
		let hz = hz.unwrap_or(48_000.0);

		Self { time: 0.0, step: 1.0 / hz, pitch, state: 0.0 }
	}

	/// Change the pitch of the sound.
	#[inline(always)]
	pub fn pitch(&mut self, pitch: f64) {
		self.pitch = pitch;
	}
}

impl Iterator for Sound {
	type Item = Wave;

	#[inline(always)]
	fn next(&mut self) -> Option<Wave> {
		let wave = Wave(Sound {
			time: self.time,
			step: self.step,
			pitch: self.pitch,
			state: self.state,
		});

		self.time += self.step;
		self.state = (self.state + (self.step * self.pitch)) % 1.0;

		Some(wave)
	}
}

/// The information necessary to sample a common waveform.
pub struct Wave(Sound);

impl Wave {
	/// Sample a sawtooth wave.
	pub fn saw(&self) -> Sample {
		Sample {
			t: self.0.time,
			v: self.0.state * -2.0 + 1.0,
		}
	}

	/// Sample a square wave.
	pub fn sqr(&self) -> Sample {
		Sample {
			t: self.0.time,
			v: (self.0.state * -2.0 + 1.0).signum(),
		}
	}

	/// Sample a triangle wave.
	pub fn tri(&self) -> Sample {
		Sample {
			t: self.0.time,
			v: (self.0.state * -2.0 + 1.0).abs() * 2.0 - 1.0,
		}
	}

	/// Sample a sine wave.
	pub fn sin(&self) -> Sample {
		Sample {
			t: self.0.time,
			v: sin(self.0.state),
		}
	}

	/// Sample a harmonic series derived sound.
	pub fn har(&self, overtones: &[f64]) -> Sample {
		Sample {
			t: self.0.time,
			v: ovr(self.0.state, overtones),
		}
	}

    /// Sample a sound analyzed by FFT.
    pub fn ovr(&self, overtones: &[(f64, f64)]) -> Sample {
        Sample {
            t: self.0.time,
            v: ovr_advanced(self.0.state, overtones),
        }
    }
}

#[inline(always)] fn sin(x: f64) -> f64 {
	(x * (::std::f64::consts::PI * 2.0)).sin()
}

/// Generate sound from fundamental and overtones (reverse FFT).
#[inline(always)] fn ovr(x: f64, overtones: &[f64]) -> f64 {
	let mut o = sin(x);
	let mut v = 1.0;
	let mut d = 1.0;
	for i in overtones {
		d += 1.0;
		v += i;
		o += sin(x * d) * i;
	}
	o / v
}

/// Generate sound from fundamental and overtones (reverse FFT).
#[inline(always)] fn ovr_advanced(x: f64, overtones: &[(f64, f64)]) -> f64 {
	let mut o = sin(x);
	let mut v = 1.0;
	for (d, i) in overtones {
		v += i;
		o += sin(x * d) * i;
	}
	o / v
}
