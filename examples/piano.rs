//! A Minor on an Electric Piano

use fon::{mono::Mono64, Audio, Sink};
use twang::{Mix, Synth, Fc, Signal};

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
const VOLUME: f64 = 0.1;

fn main() {
    fn piano(_: &mut (), fc: Fc) -> Signal {
        PITCHES
            .iter()
            .map(|p| {
                HARMONICS
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        fc.freq(p * (i + 1) as f64).sine().gain(v * VOLUME)
                    })
                    .mix()
            })
            .mix()
    }

    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new((), piano);
    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "piano.wav").expect("Failed to write WAV file");
}
