use twang::{
    gen::{Triangle},
    mono::Mono64,
    ops::{Max, Min, Sine, Add, Sawtooth},
    Audio, Hz,
};

mod wav;

fn main() {
    let mut tri = Triangle::new(Hz(220.0)); // A4
    let mut note;
    let mut temp = Audio::with_silence(48_000, 48_000 * 5);
    
    temp.generate(&mut tri);
    note = Audio::with_audio(temp.sample_rate(), &temp);
    temp.blend_sample(Mono64::new(1.0), Sawtooth);
    temp.blend_sample(Mono64::new(0.0), Max);
    note.blend_sample(Mono64::new(1.0), Sine);
    note.blend_sample(Mono64::new(0.0), Min);
    note.blend_audio(&temp, Add);
    
    // Write chord to file
    wav::write(note, "organ.wav").expect("Failed to write WAV file");
}
