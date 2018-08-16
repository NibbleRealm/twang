extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use adi::speaker::Speaker;
use twang::{White};

fn main() {
	let mut speaker = Speaker::new(0, false).unwrap();
	let mut whts = White::new(None);

	loop {
		speaker.update(&mut || {
			whts.next().unwrap().into()
		});
	}
}
