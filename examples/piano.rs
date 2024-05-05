//! A Minor on an Electric Piano

use fon::{chan::Ch16, Audio, Frame};
use twang::next::{Synth, Wave};

mod wav;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [f32; 3] = [220.0, 220.0 * 32.0 / 27.0, 220.0 * 3.0 / 2.0];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

/*
// State of the synthesizer.
#[derive(Default)]
struct Processors {
    // White noise generator.
    white: White,
    // 10 harmonics for 3 pitches.
    piano: [[Sine; 10]; 3],
} */
const GAINS: &[Wave; 10] = &[
    Wave::sig(HARMONICS[0]),
    Wave::sig(HARMONICS[1]),
    Wave::sig(HARMONICS[2]),
    Wave::sig(HARMONICS[3]),
    Wave::sig(HARMONICS[4]),
    Wave::sig(HARMONICS[5]),
    Wave::sig(HARMONICS[6]),
    Wave::sig(HARMONICS[7]),
    Wave::sig(HARMONICS[8]),
    Wave::sig(HARMONICS[9]),
];

/// Play a note on the piano from a sine wave
const fn piano(sine: &'static Wave) -> [Wave<'static>; 10] {
    [
        sine.amp(&GAINS[0]),
        sine.amp(&GAINS[1]),
        sine.amp(&GAINS[2]),
        sine.amp(&GAINS[3]),
        sine.amp(&GAINS[4]),
        sine.amp(&GAINS[5]),
        sine.amp(&GAINS[6]),
        sine.amp(&GAINS[7]),
        sine.amp(&GAINS[8]),
        sine.amp(&GAINS[9]),
    ]
}

fn main() {
    // Define waveform
    const FIRST: Wave = Wave::mix(&piano(&Wave::sig(PITCHES[0]).sine()));
    const THIRD: Wave = Wave::mix(&piano(&Wave::sig(PITCHES[1]).sine()));
    const FIFTH: Wave = Wave::mix(&piano(&Wave::sig(PITCHES[2]).sine()));
    const PIANO: Wave =
        Wave::mix(&[FIRST, THIRD, FIFTH]).amp(&Wave::sig(VOLUME));

    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(PIANO);

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);

    // Plot synthesized audio, and write to a WAV file
    // plot::write(&audio);
    wav::write(audio, "piano.wav").expect("Failed to write WAV file");

    /*
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let mut proc = Processors::default();
    // Adjust phases of harmonics.
    for pitch in proc.piano.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, mut frame: Frame<_, 2>| {
        for (s, pitch) in proc.piano.iter_mut().zip(PITCHES.iter()) {
            for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                // Get next sample from oscillator.
                let sample = o.step(pitch * (i + 1) as f32);
                // Pan the generated harmonic center
                frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
            }
        }
        frame
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "piano.wav").expect("Failed to write WAV file"); */
}
