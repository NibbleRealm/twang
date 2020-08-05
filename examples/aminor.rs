extern crate adi;
extern crate twang; // for sound generation / effects // for speaker

use adi::speaker::Speaker;
use twang::{prelude::*, Sound};

fn main() {
    let mut speaker = Speaker::new(0, false).unwrap();
    let piano = [
        0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
    ];
    let mut gen = Sound::new(None, 220.0); // A3
    let mut gen2 = Sound::new(None, 220.0 * 32.0 / 27.0); // third
    let mut gen3 = Sound::new(None, 220.0 * 3.0 / 2.0); // fifth

    loop {
        speaker.update(&mut || {
            // Play A Minor Chord on Synthesized Piano
            let x1 = gen.next().unwrap().har(&piano);
            let x2 = gen2.next().unwrap().har(&piano);
            let x3 = gen3.next().unwrap().har(&piano);

            [x1, x2, x3].mix().into()
        });
    }
}
