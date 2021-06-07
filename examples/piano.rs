//! A Minor on an Electric Piano

use fon::chan::Ch16;
use fon::{Audio, Frame, Stream};
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;

mod wav;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    // White noise generator.
    white: White,
    // 10 harmonics for 3 pitches.
    piano: [[Sine; 10]; 3],
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::new(48_000);
    // Create audio processors
    let mut proc = Processors::default();
    // Adjust phases of harmonics.
    for pitch in proc.piano.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.next());
        }
    }
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 2>| {
        for (s, pitch) in proc.piano.iter_mut().zip(PITCHES.iter()) {
            for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                // Get next sample from oscillator.
                let sample = o.next(pitch * (i + 1) as f32);
                // Pan the generated harmonic center
                frame = frame.pan(Gain.next(sample, (v * VOLUME).into()), 0.0);
            }
        }
        frame
    });
    // Synthesize 5 seconds of audio
    synth.extend(&mut audio, 48_000 * 5);
    // Write synthesized audio to WAV file
    wav::write(audio, "piano.wav").expect("Failed to write WAV file");
}
