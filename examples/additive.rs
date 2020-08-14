use twang::Fc;
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
        let fundamental = fc.freq(440.0).sine();
        *sample = fundamental.into();
    }

    // Write chord to file
    wav::write(audio, "additive.wav").expect("Failed to write WAV file");
}
