use fon::{mono::Mono64, Audio, Sink};
use twang::{Pink, Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the white noise generator.
    let mut pink = Pink::new();
    // Create the synthesizer.
    let mut synth = Synth::new(|_fc| pink.noise());

    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "pink.wav").expect("Failed to write WAV file");
}
