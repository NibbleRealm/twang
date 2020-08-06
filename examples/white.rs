use twang::{Audio, gen::White, mono::Mono64};

mod wav;

fn main() {
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut pink = White::new();
    out.generate(&mut pink);
    wav::write(out, "white.wav").expect("Failed to write WAV file");
}
