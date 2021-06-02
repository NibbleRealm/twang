use fon::{chan::Ch64, Audio, Stream};
use twang::{Fc, Signal, Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn sine(_: &mut (), fc: Fc) -> Signal {
    fc.freq(440.0).sine()
}

fn main() {
    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new((), sine);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);
    // Write synthesized audio to WAV file.
    wav::write(audio, "sine.wav").expect("Failed to write WAV file");
}
