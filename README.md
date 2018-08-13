[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)

# Twang
A sound synthesis crate.

## Features
* A bunch of sound synthesization functions.

## Example
```rust
extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use twang as t;

fn main() {
	let mut speaker = adi::speaker::Speaker::new(0, false).unwrap();
	let mut gen = t::Generator::new(440.0, 1.0);

	loop {
		let x = gen.next();

		// Play synthesized voice.
		speaker.update(&mut || {
			// Do synthesis
			t::out(t::mul(&[
				t::dst(t::sin(x), 2),
				t::dst(t::saw(x), 2)
			]))
		});
	}
}
```

## Roadmap to 1.0 (Future Features)
WIP

## Change Log
### 0.1
* First release
