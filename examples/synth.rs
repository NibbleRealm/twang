//! # Frequency modulation (FM) synthesis
//!
//! Variables:
//! - Output Amplitude (e)
//! - Carrier Frequency Signal (cc)
//! - Modulation Frequency Signal (mm)
//! - Time-Varying Modulation Index Signal (I)
//!
//! ```
//! e = sin(cc * t + I(t) sin(mm * t))
//! ```
//!
//! # Phase modulation (PM) synthesis
//! To generate a sine wave modulating the carrier sine wave:
//! ```
//! let message = fc.freq(660.0).sine()
//! let carrier = fc.freq(440.0).shift(message).sine()
//! ```

use fon::{mono::Mono64, Audio, Sink};
use twang::{Synth, Fc, Signal};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn gen_synth(_: &mut (), fc: Fc) -> Signal {
        let freq_modulator: f64 = fc.freq(880.0).sine().into();
        let norm_modulator = (freq_modulator + 1.0) * 0.5;
        fc.freq(220.0 * norm_modulator).sine().gain(0.7)
    }

    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new((), gen_synth);

    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "synth.wav").expect("Failed to write WAV file");
}
