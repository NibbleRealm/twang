use twang::{Audio, gen::Pink, mono::Mono64};

mod wav;

fn main() {
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut pink = Pink::new();
    out.generate(&mut pink);
    wav::write(out, "pink.wav").expect("Failed to write WAV file");
}
