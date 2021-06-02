use fon::{chan::Ch64, Audio, Stream};
use twang::{Mix, Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn organ(_: &mut (), fc: Fc) -> Signal {
        let pt_a = fc.freq(220.0).triangle().max(0.0);
        let pt_b = fc.freq(220.0).sine().min(0.0);
        [pt_a, pt_b].mix()
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new((), organ);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);

    // Write synthesized audio to WAV file.
    wav::write(audio, "organ.wav").expect("Failed to write WAV file");
}
