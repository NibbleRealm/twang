use fon::{chan::Ch16, Audio, Frame};
use twang::osc::Sine;
use twang::Synth;

mod wav;

// State of the synthesizer.
struct Processors {
    sine: Sine,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let proc = Processors { sine: Sine::new() };
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let sine = proc.sine.step(440.0);
        // Pan the generated audio center
        frame.pan(sine, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
