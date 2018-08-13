# Twang
A sound synthesis crate.

```rust
extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use twang as t;

fn main() {
	let mut audio = adi::speaker::AudioManager::new();
	let mut gen = t::Generator::new(440.0, 1.0);

	loop {
		// Play synthesized voice.
		audio.play(&mut || gen.gen(&mut |x| {
			// Do synthesis
			t::mul(&[
				t::dst(t::sin(x), 2),
				t::dst(t::saw(x), 2)
			])
		}));
	}
}
```
