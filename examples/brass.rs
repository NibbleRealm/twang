use twang::{
    gen::{Pink, Saw},
    mono::Mono64,
    ops::{Abs, Add, ClipHard, Gain, Max, Triangle},
    Audio, Hz,
};

mod wav;

fn main() {
    let mut pink = Pink::new();

    // Five seconds of 48 KHz Audio
    let mut note = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut temp = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    let mut tmp2 = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);

    // Add airy brass noise
    let mut saw = Saw::new(Hz(220.0)); // A4
    temp.generate(&mut saw);
    temp.blend_sample(Mono64::new(0.075), ClipHard);
    tmp2.generate(&mut pink);
    tmp2.blend_sample(Mono64::new(1.0), Abs);
    temp.blend_audio(&tmp2, Max);
    temp.blend_sample(Mono64::new(0.75), Gain);
    note.blend_audio(&temp, Add);

    // Add the main sound
    saw = Saw::new(Hz(220.0)); // A4
    temp.generate(&mut saw);
    temp.blend_sample(Mono64::new(0.075), ClipHard);
    tmp2 = Audio::with_audio(temp.sample_rate(), &temp);
    tmp2.blend_sample(Mono64::new(1.0), Abs);
    temp.blend_sample(Mono64::new(1.0), Triangle);
    temp.blend_audio(&tmp2, Gain);
    note.blend_audio(&temp, Add);

    // Write chord to file
    wav::write(note, "brass.wav").expect("Failed to write WAV file");
}
