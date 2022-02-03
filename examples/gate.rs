//! "Gated Reverb" example to recreate the "Gated Drums" effect from Phil
//! Collins' "In  The Air Tonight".
//!
//! Steps:
//!  1. Load original audio (a dry percussive sound).
//!  2. Generate large reverb reflections with `Room` (may also add / substitute
//!     other processing, like a compressor).
//!  3. Use noise gate with `Gate`, setting `key` to orginal audio and `input`
//!     to the reverb audio.  Set `attack` to 0.0, `hold` to 0.5, and `release`
//!     to 0.005 (these are recommendations, can be tweaked)
//!  4. Mix reverb audio with original audio.
//!
//! You need raw PCM F32LE Audio at 48KHz to run this example.

use fon::pos::{Left, Right};
use fon::chan::{Ch16, Ch32};
use fon::{Audio, Frame};
use twang::ops::{Gate, Room, GateParams, Gain};
use twang::Synth;
use std::convert::TryInto;

mod wav;

// State of the synthesizer.
struct Processors<'a> {
    input: Box<dyn Iterator<Item = Frame<Ch32, 2>> + 'a>,
    room: [Room; 2],
    gate: [Gate; 2],
}

fn main() {
    // Load audio file.
    let input = std::fs::read(
        std::env::args().skip(1).next().expect("Need a PCM file"),
    )
    .expect("Failed to read file");
    let mut buffer = Vec::new();
    for bytes in input.chunks(4) {
        buffer.push(f32::from_le_bytes(bytes.try_into().unwrap()));
    }
    let input = Audio::<Ch32, 2>::with_f32_buffer(48_000, buffer);

    // Initialize output audio (input audio length plus hold time)
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, input.len() + 24_000);

    // Create audio processors
    let proc = Processors {
        input: Box::new(input.iter().cloned()),
        room: [Room::new(), Room::new()],
        gate: [Gate::new(); 2],
    };

    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Get input sample.
        let dry = proc.input.next().unwrap_or(Default::default());

        let mut wet = [proc.room[0].step(), proc.room[1].step()];
        let dry = [dry[Left], dry[Right]];
        for (i, gate) in proc.gate.iter_mut().enumerate() {
            wet[i] = gate.step(&GateParams {
                close_threshold: Ch32::new(0.25), // Quarter max amplitude
                open_threshold: Ch32::new(0.25), // Quarter max amplitude
                range: Ch32::new(1.0), // Total silence after gate closes.
                key: dry[i],
                input: wet[i],
                attack: 0.0,
                hold: 0.5,
                release: 0.005,
            });
        }

        //proc.room[0].add(dry[0], 48 * 20 /* 30 milliseconds */, 0.5);
        //proc.room[1].add(dry[1], 48 * 20 /* 30 milliseconds */, 0.5);
        
        let left = dry[0] + wet[0];
        let right = dry[1] + wet[1];

        proc.room[0].add(left, 0.002 /* 10 milliseconds */, 0.9);
        proc.room[1].add(right, 0.002 /* 10 milliseconds */, 0.9);

        let left = Gain.step(left, Ch32::new(0.75));        
        let right = Gain.step(right, Ch32::new(0.75));        

        frame.pan(left, -0.25)
            .pan(right, 0.25)
    });
    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());
    // Write synthesized audio to WAV file
    wav::write(audio, "gate.wav").expect("Failed to write WAV file");
}
