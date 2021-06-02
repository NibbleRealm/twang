use fon::{chan::Ch64, Audio, Stream};
use twang::{Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn saw(_: &mut (), fc: Fc) -> Signal {
        fc.freq(220.0)
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new((), saw);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);

    // Write synthesized audio to WAV file.
    wav::write(audio, "saw.wav").expect("Failed to write WAV file");
}
