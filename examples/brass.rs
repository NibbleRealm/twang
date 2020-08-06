use twang::{
    gen::{Pink, Triangle},
    mono::Mono64,
    ops::{Abs, Add, ClipHard, ClipSoft, Gain, Sawtooth, Sine},
    Audio, Hz,
};

mod wav;

fn main() {
    let mut tri = Triangle::new(Hz(440.0)); // A4
    let mut pink = Pink::new();

    // Five seconds of 48 KHz Audio
    let mut note = Audio::with_silence(48_000, 48_000 * 5);
    let mut temp = Audio::with_silence(48_000, 48_000 * 5);
    let mut tmp2;

    // Add the main sound
    temp.generate(&mut tri);
    tmp2 = Audio::with_audio(temp.sample_rate(), &temp);
    tmp2.blend_sample(Mono64::new(1.0), Sawtooth);
    tmp2.blend_sample(Mono64::new(1.0), Abs);
    temp.blend_sample(Mono64::new(1.0), Sine);
    temp.blend_sample(Mono64::new(0.25), ClipHard);
    temp.blend_audio(&tmp2, Gain);
    temp.blend_sample(Mono64::new(0.85), Gain);
    note.blend_audio(&temp, Add);

    // Add airy brass noise
    temp.generate(&mut tri);
    temp.blend_sample(Mono64::new(1.0), Sine);
    temp.blend_audio(&tmp2, Gain);
    tmp2.generate(&mut pink);
    temp.blend_audio(&tmp2, Gain);
    temp.blend_sample(Mono64::new(0.15), Gain);
    note.blend_audio(&temp, Add);

    // Distortion
    note.blend_sample(Mono64::new(0.25), ClipSoft);
    note.blend_sample(Mono64::new(2.0 / 3.0), ClipHard);

    // Write chord to file
    wav::write(note, "brass.wav").expect("Failed to write WAV file");
}
