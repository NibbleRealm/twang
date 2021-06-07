use fon::{chan::Ch16, Audio, Frame, Stream};
use twang::osc::Sawtooth;
use twang::Synth;

mod wav;

// State of the synthesizer.
struct Processors {
    saw: Sawtooth,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::new(48_000);
    // Create audio processors
    let proc = Processors {
        saw: Sawtooth::new(),
    };
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let saw = proc.saw.next(440.0);
        // Pan the generated audio center
        frame.pan(saw, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.extend(&mut audio, 48_000 * 5);
    // Write synthesized audio to WAV file
    wav::write(audio, "sawtooth.wav").expect("Failed to write WAV file");
}
