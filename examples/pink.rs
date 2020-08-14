use twang::gen::{Generator, Pink};
use fon::{mono::Mono64, Audio};

mod wav;

fn main() {
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut pink = Pink::new();
    pink.generate(&mut out);
    wav::write(out, "pink.wav").expect("Failed to write WAV file");
}
