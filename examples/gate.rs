//! Gated reverb example.
//!
//! You need raw PCM F32LE Audio at 48KHz to run this example.
//!
//! 1. Generate Percussive Sound
//! 2. Add Reverb
//! 3. Compress (optional)
//! 4. Apply Noise Gate To Resulting Sound (Side chaining the original sound)
//! 5. About 1/2 second hold, few ms release time envelope on Noise Gate
//! 6. Mix with original sound

use fon::{mono::Mono64, Audio, Sink};
use std::convert::TryInto;
use twang::{Mix, Room, Signal, Synth};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;
const MILLIS: u32 = S_RATE / 1_000;

const HOLD_TIME: u32 = 150 * MILLIS;
const DECAY_TIME: u32 = 150 * MILLIS;
const REVERB: f64 = 0.75;
const ECHO_TIME: u32 = 10 * MILLIS;

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(
        S_RATE,
        (HOLD_TIME + DECAY_TIME).try_into().unwrap(),
    );
    let input = std::fs::read(
        std::env::args().skip(1).next().expect("Need a PCM file"),
    )
    .expect("Failed to read file");
    let mut input = input.chunks_exact(8).map(|sample| {
        f64::from_le_bytes([
            sample[0], sample[1], sample[2], sample[3], sample[4], sample[5],
            sample[6], sample[7],
        ])
    });

    // Set up room for max 20 milliseconds reverb.
    let mut room = Room::new((ECHO_TIME).try_into().unwrap());
    // Set counter to zero.
    let mut counter = 0;

    // Create the synthesizer.
    let mut synth = Synth::new(|_fc| {
        // 1. Percussive Sound
        let orig: Signal = input.next().unwrap_or(0.0).into();
        // 2. Add Reverb
        room.add(orig, (ECHO_TIME).try_into().unwrap(), REVERB);
        let reverb = room.gen();
        room.add(reverb, (ECHO_TIME - 1).try_into().unwrap(), REVERB);
        // 3. Compress (FIXME)
        let compressed = reverb;
        // 4. Apply Noise Gate (Side Chain Original Sound)
        let gated = compressed.gate(orig);
        // 5. Envelope half second few ms release on gate
        let out = if counter > HOLD_TIME {
            let attn = (counter - HOLD_TIME) as f64 / DECAY_TIME as f64;
            gated.gain(1.0 - attn)
        } else {
            gated
        };
        // 6. Blend With Original Sound
        let fina1 = [orig, out].mix().gain(0.5);

        // ---
        counter += 1;
        //fina1
        fina1
    });

    // Synthesize Original With Gated Reverb
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "gated.wav").expect("Failed to write WAV file");
}
