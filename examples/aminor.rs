use twang::{
    gen::Saw,
    mono::Mono64,
    ops::{Add, Sine},
    Audio, Hz,
};

mod wav;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f64; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f64; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];

fn main() {
    let mut gen;

    // Five seconds of 48 KHz Audio
    let mut chord = Audio::with_silence(48_000, 48_000 * 5);
    let mut temp;

    // Synthesize an A minor chord.
    let volume = 0.25; // To avoid clipping
    for pitch in PITCHES.iter().cloned() {
        // Add note to chord
        for (i, harmonic) in HARMONICS.iter().cloned().enumerate() {
            let i: f64 = (i as i32).into();
            gen = Saw::new(Hz(pitch * i));
            temp = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
            temp.generate(&mut gen);
            temp.blend_sample(Mono64::new(harmonic * volume), Sine);
            // Add harmonic to chord
            chord.blend_audio(&temp, Add);
        }
    }

    // Write chord to file
    wav::write(chord, "aminor.wav").expect("Failed to write WAV file");
}
