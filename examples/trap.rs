use fon::{chan::Ch16, Audio, Frame};
use twang::next::{Synth, Wave};

mod wav;

fn main() {
    // Define waveform
    const TRAP: Wave = Wave::sig(440.0).trap(&Wave::sig(0.5), Wave::ZERO);

    // Initialize audio, and create synthesizer
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48_000 * 5);
    let mut synth = Synth::new(TRAP);

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink(), &[]);

    // Write synthesized audio to WAV file
    wav::write(audio, "trap.wav").expect("Failed to write WAV file");
}
