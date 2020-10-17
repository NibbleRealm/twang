use fon::{mono::Mono64, Audio};
use twang::{Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new();
    // Generate audio samples.
    synth.gen(audio.sink(..), |fc| {
        fc.freq(220.0).sine()
    });

    // Write synthesized audio to WAV file.
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
