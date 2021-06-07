use fon::{chan::Ch16, Audio, Frame, Stream};
use twang::noise::White;
use twang::Synth;

mod wav;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    white: White,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::new(48_000);
    // Create audio processors
    let proc = Processors::default();
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let noise = proc.white.next();
        // Pan the generated audio center
        frame.pan(noise, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.extend(&mut audio, 48_000 * 5);
    // Write synthesized audio to WAV file
    wav::write(audio, "white.wav").expect("Failed to write WAV file");
}
