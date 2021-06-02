use fon::{chan::Ch64, Audio, Stream};
use twang::{Mix, Pink, Synth, Signal, Fc};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn brass(pink: &mut Pink, fc: Fc) -> Signal {
        let pink = pink.noise();
        let tone = fc.freq(220.0).gain(12.0).clamp().gain(0.1);
        let airy = tone.abs().gain(pink.abs());

        let pone = fc.freq(220.0).gain(12.0).clamp().abs();
        let ptwo = fc.freq(220.0).triangle();
        let main = pone.gain(ptwo);

        [airy, main].mix()
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new(Pink::new(), brass);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);

    // Write synthesized audio to WAV file.
    wav::write(audio, "brass.wav").expect("Failed to write WAV file");
}
