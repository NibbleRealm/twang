use fon::{chan::Ch16, Audio, Frame};
use twang::tree::{Synth, Sine, Hz, Wave};

mod wav;

// Define waveform
const fn waveform() -> impl Wave {
    Sine(Hz(440.0))
}

fn main() {
    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(waveform());

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);

    // Plot synthesized audio, and write to a WAV file
    // plot::write(&audio);
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
