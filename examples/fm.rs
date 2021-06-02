//! # Frequency modulation (FM) synthesis
//! Implemented with "Phase Modulation" algorithm.

use fon::{chan::Ch64, Audio, Stream};
use twang::{Synth, Fc, Signal};

use core::f64::consts::TAU;

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn gen_synth(t: &mut f64, _fc: Fc) -> Signal {
        // Makes 220 hz signal
        let hz = 220.0;
        let modulating = (hz * 1.5 * TAU * *t).sin();
        let carrier = (hz * TAU * *t + modulating).sin();
        *t = *t + 1.0 / S_RATE as f64;
        Signal::from(carrier)
    }

    // Initialize audio.
    let mut audio = Audio::<Ch64, 1>::new(S_RATE);
    // Create the synthesizer.
    let mut synth = Synth::new(0.0, gen_synth);
    // Stream 5 seconds of synth into audio buffer.
    synth.extend(&mut audio, S_RATE as usize * 5);

    // Write synthesized audio to WAV file.
    wav::write(audio, "fm.wav").expect("Failed to write WAV file");
}
