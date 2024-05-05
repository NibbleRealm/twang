use fon::{chan::Ch16, Audio};
use twang::tree::{line::Line, Synth};

mod wav;
//mod plot;

fn main() {
    // Define waveform
    let waveform = const { Line(440.0).osc().bezier(Line(1.0)) };
    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(waveform);

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);
    // Plot synthesized audio, and write to a WAV file
    //plot::write(&audio);
    wav::write(audio, "bezier.wav").expect("Failed to write WAV file");
}
