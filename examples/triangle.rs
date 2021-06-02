use fon::{chan::Ch64, Audio, Stream};
use twang::{Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn triangle(_: &mut (), fc: Fc) -> Signal {
        fc.freq(220.0).triangle().gain(0.7)
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new((), triangle);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);
    // Write synthesized audio to WAV file.
    wav::write(audio, "triangle.wav").expect("Failed to write WAV file");
}
