[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)

# Twang
A sound synthesis crate.

## Features
* A bunch of sound synthesization functions.

## A4 (440 Hz) Organ Example
```rust
extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use adi::speaker::Speaker;
use twang::Sound;

fn main() {
	let mut speaker = Speaker::new(0, false).unwrap();
	let mut snds = Sound::new(None, 440.0); // A4

	loop {
		speaker.update(&mut || {
			let x = snds.next().unwrap();

			(x.sin().pos() + x.tri().neg()).into()
		});
	}
}
```

## Roadmap to 1.0 (Future Features)
* WIP

## Change Log
### 0.2
* Newtype'd everything.
* Uses operator overloading now.

### 0.1
* First release
