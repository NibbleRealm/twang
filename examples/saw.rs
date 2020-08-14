use twang::gen::{Generator, Saw};
use fon::{mono::Mono64, Audio, Hz};

mod wav;

fn main() {
    let mut saw = Saw::new(Hz(220.0)); // A4
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    saw.generate(&mut out);
    wav::write(out, "saw.wav").expect("Failed to write WAV file");
}
