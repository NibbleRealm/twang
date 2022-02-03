use fon::chan::{Ch16, Ch32};
use fon::{Audio, Frame};
use twang::ops::Gain;
use twang::osc::{Sawtooth, Sine};
use twang::Synth;

mod wav;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    saw: Sawtooth,
    sin: Sine,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let mut proc = Processors::default();
    // Shift sawtooth wave
    proc.saw.shift(Ch32::new(0.25));
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let saw = proc.saw.step(440.0);
        let sin = proc.sin.step(440.0);
        // Control the gain of the sine wave with the sawtooth wave.
        let voice = Gain.step(sin, saw);
        // Pan the generated audio center
        frame.pan(voice, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "voice.wav").expect("Failed to write WAV file");
}
