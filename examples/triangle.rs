use fon::{mono::Mono64, Audio};
use twang::Fc;

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Generate five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Set up the frequency counter.
    let fc = Fc::new(S_RATE);

    // Tree-style synthesis
    for (sample, fc) in audio.iter_mut().zip(fc) {
        *sample = fc.freq(220.0).triangle().amp(0.7).to_mono()
    }

    wav::write(audio, "triangle.wav").expect("Failed to write WAV file");
}
