use twang::{
    gen::{Triangle},
    mono::Mono64,
    ops::Sine,
    Audio, Hz,
};

mod wav;

fn main() {
    let mut tri = Triangle::new(Hz(220.0)); // A4
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    out.generate(&mut tri);
    out.blend_sample(Mono64::new(1.0), Sine);
    wav::write(out, "sine.wav").expect("Failed to write WAV file");
}
