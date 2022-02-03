use fon::chan::{Ch16, Ch32};
use fon::{Audio, Frame};
use twang::ops::{Max, Min};
use twang::osc::{Sine, Triangle};
use twang::Synth;

mod wav;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    tri: Triangle,
    sin: Sine,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let proc = Processors::default();
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let tri = proc.tri.step(440.0);
        let sin = proc.sin.step(440.0);
        // Positive waveform is triangle, negative is sine.
        let out = Max.step(tri, Ch32::new(0.0)) + Min.step(sin, Ch32::new(0.0));
        // Pan the generated audio center
        frame.pan(out, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "organ.wav").expect("Failed to write WAV file");
}
