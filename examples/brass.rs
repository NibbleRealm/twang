use fon::chan::{Ch16, Ch32};
use fon::{Audio, Frame};
use twang::noise::Pink;
use twang::ops::{Clip, Gain};
use twang::osc::{Sawtooth, Triangle};
use twang::Synth;

mod wav;

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    pink: Pink,
    tone: Sawtooth,
    ptri: Triangle,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    // Create audio processors
    let proc = Processors::default();
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let pink = proc.pink.step();
        let tone = proc.tone.step(440.0);
        let ptri = proc.ptri.step(440.0);

        //
        let tone = Clip.step(tone, Ch32::new(1.0 / 12.0));
        let airy = Gain.step(Gain.step(tone, Ch32::new(12.0 / 10.0)), pink);
        let main = Gain.step(ptri, Gain.step(tone, Ch32::new(12.0)));

        frame.pan(airy, 0.0).pan(main, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "brass.wav").expect("Failed to write WAV file");
}
