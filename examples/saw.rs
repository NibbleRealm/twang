use fon::{mono::Mono64, Audio, Sink};
use twang::{Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn saw(_: &mut (), fc: Fc) -> Signal {
        fc.freq(220.0)
    }

    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new((), saw);

    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "saw.wav").expect("Failed to write WAV file");
}
