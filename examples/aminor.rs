use twang::{Audio, Hz, ops::{Sine, Add}, gen::Triangle, chan::Ch64};

mod wav;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f64; 10] = [0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f64; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];

fn main() {
    let mut gen = Triangle::new(Hz(220.0));

    // Five seconds of 48 KHz Audio
    let mut chord = Audio::with_silence(48_000, 48_000 * 5);
    let mut note;
    let mut temp;

    // Synthesize an A minor chord.
    for pitch in PITCHES.iter().cloned() {
        note = Audio::with_silence(48_000, 48_000 * 5);
        for (i, harmonic) in HARMONICS.iter().cloned().enumerate() {
            gen = Triangle::new(Hz(pitch * i.into()));
            temp = Audio::with_silence(48_000, 48_000 * 5);
            temp.generate(&mut gen);
            temp.blend_sample(Ch64::new(harmonic), Sine);
            // Add harmonic to note
            note.blend_audio(&temp, Add);
        }
        // Add note to chord
        chord.blend_audio(&note, Add);
    }

    // Write chord to file
    wav::write(chord, "aminor.wav");
}
