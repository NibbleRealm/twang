use fon::{chan::Ch16, Audio, Frame, Stream};
use twang::osc::Sine;
use twang::Synth;

mod wav;

// State of the synthesizer.
struct Processors {
    sine: Sine,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::new(48_000);
    // Create audio processors
    let proc = Processors { sine: Sine::new() };
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let sine = proc.sine.next(440.0);
        // Pan the generated audio center
        frame.pan(sine, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.extend(&mut audio, 48_000 * 5);
    // Write synthesized audio to WAV file
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
