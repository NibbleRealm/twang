use twang::{
    gen::{Generator, Saw},
    mono::Mono64,
    ops::{Add, Sine, Gain},
    sample::Sample,
    Audio, Hz,
};

mod wav;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f64; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// The three pitches in a perfectly tuned A3 minor chord
const PITCHES: [Hz; 3] = [Hz(220.0), Hz(220.0 * 32.0 / 27.0), Hz(220.0 * 3.0 / 2.0)];
const VOLUME: f64 = 0.25;

struct Piano {
    harmonics: [Saw; 10],
}

impl Piano {
    pub fn new(pitch: Hz) -> Self {
        let harmonics: [Saw; 10] = [
            Saw::new(Hz(pitch.0 * 1.0)),
            Saw::new(Hz(pitch.0 * 2.0)),
            Saw::new(Hz(pitch.0 * 3.0)),
            Saw::new(Hz(pitch.0 * 4.0)),
            Saw::new(Hz(pitch.0 * 5.0)),
            Saw::new(Hz(pitch.0 * 6.0)),
            Saw::new(Hz(pitch.0 * 7.0)),
            Saw::new(Hz(pitch.0 * 8.0)),
            Saw::new(Hz(pitch.0 * 9.0)),
            Saw::new(Hz(pitch.0 * 10.0)),
        ];
        Piano {
            harmonics
        }
    }
}

impl Generator for Piano {
    fn sample(&mut self, delta: std::time::Duration) -> Mono64 {
        let mut ret = Mono64::new(0.0);
        for i in 0..10 {
            let mut a = self.harmonics[i].sample(delta);
            a.blend(&Mono64::new(1.0), Sine);
            a.blend(&Mono64::new(VOLUME * HARMONICS[i]), Gain);
            ret.blend(&a, Add);
        }
        ret
    }
}

fn main() {
    // The three notes that make up the chord.
    let mut piano = [Piano::new(PITCHES[0]), Piano::new(PITCHES[1]),
        Piano::new(PITCHES[2])];

    // Five seconds of 48 KHz Audio
    let mut chord = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);

    // Synthesize each sample in an A minor chord.
    for sample in chord.iter_mut() {
        // Synthesize each note in A minor chord
        for pitch in piano.iter_mut() {
            sample.blend(&pitch.sample(std::time::Duration::new(1, 0) / 48_000), Add);
        }
    }
    
    // Write chord to file
    wav::write(chord, "aminor.wav").expect("Failed to write WAV file");
}
