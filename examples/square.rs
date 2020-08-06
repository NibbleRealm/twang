use twang::{gen::Saw, mono::Mono64, ops::Square, Audio, Hz};

mod wav;

fn main() {
    let mut saw = Saw::new(Hz(220.0)); // A4
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    out.generate(&mut saw);
    out.blend_sample(Mono64::new(1.0), Square);
    wav::write(out, "square.wav").expect("Failed to write WAV file");
}
