extern crate adi;
extern crate twang; // for sound generation / effects // for speaker

use adi::speaker::Speaker;
use twang::{prelude::*, Pink, Sound};

fn main() {
    let mut speaker = Speaker::new(0, false).unwrap();
    let mut snds = Sound::new(None, 440.0); // A4
    let mut pnks = Pink::new(None);

    loop {
        speaker.update(&mut || {
            let x = snds.next().unwrap();
            let pnk = pnks.next().unwrap();
            let sin = x.sin();
            let saw = x.saw();
            let tmp = [sin.hrd(4.0), saw].mul() * 0.85 + [sin, saw, pnk].mul() * 0.15;

            tmp.sft(4.0).hrd(1.5).into()
        });
    }
}
