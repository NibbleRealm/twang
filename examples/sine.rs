use fon::{chan::Ch16, Audio, Frame};
use twang::next::{Synth, Wave};

mod wav;

fn main() {
    // Define waveform
    const SINE: Wave = Wave::sig(440.0).sine();

    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(SINE);

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);

    // Plot synthesized audio, and write to a WAV file
    // plot::write(&audio);
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
