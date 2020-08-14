use twang::gen::{Generator, White};
use fon::{mono::Mono64, Audio};

mod wav;

fn main() {
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut white = White::new();
    white.generate(&mut out);
    wav::write(out, "white.wav").expect("Failed to write WAV file");
}
