use fon::{
    mono::Mono64,
    ops::{Abs, Gain, Sine},
    Audio, Hz,
};
use twang::gen::{Generator, Saw};

mod wav;

fn main() {
    let mut saw = Saw::new(Hz(440.0)); // A4
    let mut note;
    let mut temp = Audio::with_silence(48_000, 48_000 * 5);

    saw.generate(&mut temp);
    note = Audio::with_audio(temp.sample_rate(), &temp);
    temp.blend_sample(Mono64::new(1.0), Abs);
    note.blend_sample(Mono64::new(1.0), Sine);
    note.blend_audio(&temp, Gain);

    // Write chord to file
    wav::write(note, "voice.wav").expect("Failed to write WAV file");
}
