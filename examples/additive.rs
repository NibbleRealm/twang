use twang::{Synth, Fc};
use fon::{Audio, mono::Mono64};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Generate five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Set up the frequency counter.
    let mut fc = Fc::new(S_RATE);

    // Synthesis
    for sample in audio.iter_mut() {
        fc.step();
        // Leaf Nodes
        let fundamental = fc.freq(440.0).sine(0.2);
        let harmonic_ab = fc.freq(440.0 * 2.0).sine(0.3);
        let harmonic_ac = fc.freq(440.0 * 3.0).sine(0.5);
        let harmonic_ad = fc.freq(440.0 * 4.0).sine(0.1);
        let harmonic_ae = fc.freq(440.0 * 5.0).sine(0.05);
        let harmonic_af = fc.freq(440.0 * 6.0).sine(0.02);
        // Build the synthesis tree
        let tree = [
            fundamental,
            harmonic_ab,
            harmonic_ac,
            harmonic_ad,
            harmonic_ae,
            harmonic_af,
        ].mix();
        // 
        *sample = tree.into();
    }

    // Write chord to file
    wav::write(audio, "additive.wav").expect("Failed to write WAV file");
}
