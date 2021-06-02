use fon::{chan::Ch64, Audio, Stream};
use twang::{Pink, Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn gen_pink(pink: &mut Pink, _fc: Fc) -> Signal {
        pink.noise()
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new(Pink::new(), gen_pink);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);

    // Write synthesized audio to WAV file.
    wav::write(audio, "pink.wav").expect("Failed to write WAV file");
}
