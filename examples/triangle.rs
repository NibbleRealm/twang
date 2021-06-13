use fon::{chan::Ch16, Audio, Frame};
use twang::osc::Triangle;
use twang::Synth;

mod wav;

// State of the synthesizer.
struct Processors {
    triangle: Triangle,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let proc = Processors {
        triangle: Triangle::new(),
    };
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let triangle = proc.triangle.next(440.0);
        // Pan the generated audio center
        frame.pan(triangle, 0.0)
    });
    // Synthesize 5 seconds of audio
    audio.stream(&mut synth);
    // Write synthesized audio to WAV file
    wav::write(audio, "triangle.wav").expect("Failed to write WAV file");
}
