use fon::{chan::Ch16, Audio, Frame};
use twang::tree::{osc::{Osc, Sine}, Synth, Wave};

mod wav;
//mod plot;

fn main() {
    // Define waveform
    let waveform = const { Osc(440.0).sine() };
    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(waveform);

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);
    // Plot synthesized audio, and write to a WAV file
    //plot::write(&audio);
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
