//! A Minor on an Electric Piano

use fon::{mono::Mono64, Audio};
use twang::{Fc, Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f64; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f64; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
/// Volume of the piano
const VOLUME: f64 = 0.25;

fn main() {
    // Generate five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Set up the frequency counter.
    let fc = Fc::new(S_RATE);

    // Tree-style synthesis
    for (sample, fc) in audio.iter_mut().zip(fc) {
        *sample = PITCHES
            .iter()
            .cloned()
            .map(|p| {
                HARMONICS
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(i, v)| {
                        fc.freq(p * (i + 1) as f64).sine().amp(v * VOLUME)
                    })
                    .mix()
            })
            .mix()
            .into_mono();
    }

    // Write chord to file
    wav::write(audio, "piano.wav").expect("Failed to write WAV file");
}
