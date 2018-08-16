extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use adi::speaker::Speaker;
use twang::{Pink};

fn main() {
	let mut speaker = Speaker::new(0, false).unwrap();
	let mut pnks = Pink::new(None);

	loop {
		speaker.update(&mut || {
			pnks.next().unwrap().into()
		});
	}
}
