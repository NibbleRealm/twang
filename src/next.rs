// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! # Twang File Format (Alpha)
//! The twang file format is a simple list of 32-bit instructions and data.  The
//! file must begin with magic bytes: `\xFF\xFETwAnG\0`.  Suggested file
//! extension is `.twng`.  The twang file format is designed to allow relatively
//! easy conversion into WebAssembly, and can also be used without it.  It is
//! also designed to be append-only when writing out to a file.  The final
//! instruction represents the root node of the synthesis tree.
//!
//! The layout of an instruction is as follows:
//!
//! | Byte 1 | Byte 2 | Byte 3 | Byte 4 |
//! |--------|--------|--------|--------|
//! | Index1 | Index2 | Index3 | Opcode |
//!
//! Index1-3 are the low-order bits of a 32-bit little endian unsigned offset.
//! To expand to 32-bit little-endian, AND byte 4 with `0x00`.
//!
//! Indices are backreferences in the file, so the index 0 would reference
//! itself, and 1 the byte before.  Nodes can't actually reference themselves,
//! though.  Here's a list of specially handled indices:
//!
//!  - 0: +0 Signal
//!  - 1: +1 Signal
//!  - 2: -0 Signal
//!  - 3: -1 Signal
//!
//! All references must be divisible by 4 so that they are aligned.
//!
//! ## Opcodes
//!
//! ### 0 - SIG
//!  - `index` points to 32-bit floating point LE data.
//!
//! ### 1 - MIX
//!  - `index_a` points to audio node A
//!  - `index_b` points to audio node B
//!  - `opcode` Set to 1 / TRUE for continue, 0 / FALSE to end.
//!
//! Example instruction:
//!
//! | Byte 1 | Byte 2 | Byte 3 | Byte 4 | Byte 5 | Byte 6 | Byte 7 | Byte 8 |
//! |--------|--------|--------|--------|--------|--------|--------|--------|
//! | Index1 | Index2 | Index3 | 1      | Index1 | Index2 | Index3 | FALSE  |
//!
//! ### 2 - SIN
//!  - `index` points to input node to determine hz.
//!
//! ### 3 - PHO
//!  - `index` points to input node to set phase offset before using oscillator.
//!
//! ### 4 - RMP
//!  - `index_a` points to input node to determine hz.
//!
//! ### 5 - BEZ
//!  - `index_a` points to input node to determine hz.
//!  - `index_b` points to input node to determine curve.
//!  - `opcode_b` must be 0 / FALSE.
//!
//! ### 6 - SQR
//!  - `index_a` points to input node to determine hz.
//!
//! ### 7 - PUL
//!  - `index_a` points to input node to determine hz.
//!  - `index_b` points to input node to determine duty cycle.
//!  - `opcode_b` can be 0 / FALSE, or 1 / TRUE for additional parameter.
//!  - `index_c` points to input node to determine alias (default -1).
//!  - `opcode_c` must be 0 / FALSE.
//!
//! ### 8 - LET
//!  - `index` points to input node to cache.
//!
//! ### 9 - DEL
//!  - `index` points to input node to un-cache.
//!
//! ### 10 - VAR
//!  - `index` points to user input data
//!
//! | Byte 1 | Byte 2 | Byte 3 | Byte 4 |
//! |--------|--------|--------|--------|
//! | Input1 | Input2 | Input3 | Clamp  |
//!
//!  - `input` is a unique identifier fo the input.  Range is -1 to 1.
//!  - `clamp` is a boolean indicating whether or not the user input should be
//!    clamped to be in range.
//!
//! ### 11 - CLP
//!  - `index_a` points to input node to clip by clamping from -1 to 1.
//!
//! ### 12 - MUL
//!  - `index_a` points to input node of primary wave.
//!  - `index_b` points to input node of secondary wave.
//!  - `opcode_b` can be 0 to end instruction, or 1 to multiply by more waves.
//!
//! ### 13 - MUZ
//!  - `index_a` points to input node of primary wave.
//!  - `index_b` points to input node of secondary wave.
//!  - `opcode_b` can be 0 to end instruction, or 1 to multiply by more waves.
//!
//! ### 14 - WHT
//!  - `index` points to input node for random key.
//!
//! ### 15 - PNK
//!  - `index` points to input node for random key.
//!
//! ### 16 - MIN
//!  - `index` points to input node.
//!
//! ### 17 - MAX
//!  - `index` points to input node.

use fon::Sink;

use alloc::vec::Vec;

/// A synthesis instruction
#[repr(u8)]
enum Inst {
    /// Constant signal
    Sig = 0,
    /// Mix (add) audio together
    Mix = 1,
    /// Sine wave (default)
    Sin = 2,
    /// Phase offset modifier
    Pho = 3,
    /// Ramp wave
    Rmp = 4,
    /// Ramp wave with bezier curve
    Bez = 5,
    /// Square wave
    Sqr = 6,
    /// Pulse wave
    Pul = 7,
    /// Store audio buffer
    Let = 8,
    /// Clear audio buffer - audio buffers should be cleared early, and none
    /// should be left at the root node.
    Del = 9,
    /// Define user input node
    Var = 10,
    /// Clip by clamping audio from -1 to 1
    Clp = 11,
    /// Multiply waves treating -1 as ground
    Mul = 12,
    /// Multiply waves treating 0 as ground
    Amp = 13,
    /// White noise
    Wht = 14,
    /// Pink noise
    Pnk = 15,
    /// Minimum amplitude
    Min = 16,
    /// Maximum amplitude
    Max = 17,
}

/// A synthesis file
#[derive(Debug, Copy, Clone)]
enum File<'a> {
    /// Constant signal
    Sig(f32),
    /// Mix (add) audio together
    Mix(&'a [u8]),
    /// Sine wave
    Sine {
        /// Frequency of the waveform
        hz: &'a [u8],
    },
    /// Ramp wave
    Ramp {
        /// Frequency of the waveform
        hz: &'a [u8],
        /// Amount of bezier curving to apply; set to 0 for sawtooth wave
        curve: &'a [u8],
    },
    /// Pulse wave
    Pulse {
        /// Frequency of the waveform
        hz: &'a [u8],
        /// Duty cycle; set to 0 for square wave
        duty: &'a [u8],
        /// Alias; set to -1 for no aliasing (pure pulse wave), set to 0 for
        /// trapazoid wave, and 1 for triangle wave
        alias: &'a [u8],
    },
}

/// A synthesis node
#[derive(Debug, Copy, Clone)]
enum Node<'a> {
    /// Constant signal
    Sig(f32),
    /// Mix (add) audio together
    Mix(&'a [Node<'a>]),
    /// Sine wave
    Sine {
        /// Frequency of the waveform
        hz: &'a Node<'a>,
    },
    /// Ramp wave
    Ramp {
        /// Frequency of the waveform
        hz: &'a Node<'a>,
        /// Amount of bezier curving to apply; set to 0 for sawtooth wave
        curve: &'a Node<'a>,
    },
    /// Pulse wave
    Pulse {
        /// Frequency of the waveform
        hz: &'a Node<'a>,
        /// Duty cycle; set to 0 for square wave
        duty: &'a Node<'a>,
        /// Alias; set to -1 for no aliasing (pure pulse wave), set to 0 for
        /// trapazoid wave, and 1 for triangle wave.
        alias: &'a Node<'a>,
    },

    /// Multiply audio nodes together
    Mul(&'a [Node<'a>]),
    /// Ground zero multiply
    Amp(&'a Node<'a>, &'a Node<'a>),

    /// Source from Twang file data
    File(&'a [u8]),
}

impl Node<'_> {
    /// Recursively count the number of oscillators
    fn count_osc(&self) -> usize {
        use Node::*;
        match *self {
            Sig(_) => 0,
            Mix(nodes) => {
                let mut count = 0;
                for node in nodes {
                    count += node.count_osc();
                }
                count
            }
            Sine { hz } => 1 + hz.count_osc(),
            Ramp { hz, curve } => 1 + hz.count_osc() + curve.count_osc(),
            Pulse { hz, duty, alias } => {
                1 + hz.count_osc() + duty.count_osc() + alias.count_osc()
            }
            Mul(nodes) => {
                let mut count = 0;
                for node in nodes {
                    count += node.count_osc();
                }
                count
            }
            Amp(a, b) => a.count_osc() + b.count_osc(),
            File(_) => todo!(),
        }
    }
}

/// A parameterized waveform.
///
/// For all oscillators, there is a built-in phase offset so that the first
/// output is +1.
#[derive(Debug)]
#[repr(transparent)]
pub struct Wave<'a>(Node<'a>);

impl Wave<'static> {
    /// Minimum unclipped signal (-1)
    pub const MIN: &'static Self = &Self::sig(-1.0);
    /// No signal (0), can be used for silence
    pub const ZERO: &'static Self = &Self::sig(0.0);
    /// Maximum unclipped signal (+1)
    pub const MAX: &'static Self = &Self::sig(1.0);

    /// Constant signal
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Generate silence
    /// const WAVE: Wave = Wave::sig(0.0);
    /// ```
    pub const fn sig(value: f32) -> Self {
        Self(Node::Sig(value))
    }
}

impl<'a> Wave<'a> {
    /// Load Twang file
    pub const fn file(bytes: &'a [u8]) -> Self {
        Self(Node::File(bytes))
    }

    /// Sine wave
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Sine wave at 220.0 hertz
    /// const WAVE: Wave = Wave::sig(220.0).sine();
    /// ```
    pub const fn sine(&'a self) -> Self {
        Self(Node::Sine { hz: &self.0 })
    }

    /// Sawtooth wave
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Sawtooth wave at 220.0 hertz
    /// const WAVE: Wave = Wave::sig(220.0).saw();
    /// ```
    pub const fn saw(&'a self) -> Self {
        Self(Node::Ramp {
            hz: &self.0,
            curve: &Wave::ZERO.0,
        })
    }

    /// Ramp wave (like a sawtooth wave, but can have a bezier curve).
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Ramp wave at 220.0 hertz, fast-slow-fast bezier curve
    /// const RAMP: Wave = Wave::sig(220.0).ramp(Wave::MAX);
    ///
    /// // Ramp wave at 220.0 hertz, slow-fast-slow bezier curve
    /// const EASE: Wave = Wave::sig(220.0).ramp(Wave::MIN);
    ///
    /// // Ramp wave at 220.0 hertz, no curve; same as sawtooth wave
    /// const SAWTOOTH: Wave = Wave::sig(220.0).ramp(Wave::ZERO);
    /// ```
    pub const fn ramp(&'a self, curve: &'a Self) -> Self {
        Self(Node::Ramp {
            hz: &self.0,
            curve: &curve.0,
        })
    }

    /// Square wave
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Square wave at 220.0 hertz
    /// const WAVE: Wave = Wave::sig(220.0).sq();
    /// ```
    pub const fn sq(&'a self) -> Self {
        Self(Node::Pulse {
            hz: &self.0,
            duty: &Wave::ZERO.0,
            alias: &Wave::MIN.0,
        })
    }

    /// Rectangle wave
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Rectangle wave at 220.0 hertz, 75% off - 25% on
    /// const WAVE: Wave = Wave::sig(220.0).rect(&Wave::sig(-0.5));
    /// ```
    pub const fn rect(&'a self, duty: &'a Self) -> Self {
        Self(Node::Pulse {
            hz: &self.0,
            duty: &duty.0,
            alias: &Wave::MIN.0,
        })
    }

    /// Trapazoid wave
    ///
    /// ```rust
    /// # use twang::next::Wave;
    /// // Trapazoid wave at 220.0 hz, equal duty cycle, half aliased.
    /// const TRAPAZOID: Wave = Wave::sig(220.0)
    ///     .trap(Wave::ZERO, Wave::ZERO);
    ///
    /// // Triangle wave at 220.0 hz, equal duty cycle (max alias)
    /// const TRIANGLE: Wave = Wave::sig(220.0)
    ///     .trap(Wave::ZERO, Wave::MAX);
    /// ```
    pub const fn trap(&'a self, duty: &'a Self, alias: &'a Self) -> Self {
        Self(Node::Pulse {
            hz: &self.0,
            duty: &duty.0,
            alias: &alias.0,
        })
    }

    /// Amplify an audio wave
    ///
    /// ## Warning
    /// Using this for multiplication of two audio samples is not wise, as it
    /// may double the output frequency.  If you wish to multiply waves
    /// together, use [`Wave::mul`] instead.  As long as `gain` is non-negative,
    /// this is not an issue that needs to be worried about.
    pub const fn amp(&'a self, gain: &'a Self) -> Self {
        Self(Node::Amp(&self.0, &gain.0))
    }

    /// Invert the audio wave (multiply by -1)
    pub const fn inv(&'a self) -> Self {
        Self(Node::Amp(&self.0, &Wave::MIN.0))
    }

    /// Multiply audio waves together, treating -1 as ground / zero.
    pub const fn mul(nodes: &'a [Self]) -> Self {
        Self(Node::Mul(Self::as_nodes(nodes)))
    }

    /// Mix (add) audio waves together
    pub const fn mix(nodes: &'a [Self]) -> Self {
        Self(Node::Mix(Self::as_nodes(nodes)))
    }

    /// Get node slice from wave slice
    // Safe transmute because of `repr(transparent)`
    #[allow(unsafe_code)]
    const fn as_nodes(nodes: &'a [Self]) -> &'a [Node<'a>] {
        unsafe { core::mem::transmute(nodes) }
    }
}

/// A synthesizer.
#[derive(Debug)]
pub struct Synth<'a> {
    wave: Option<Wave<'a>>,

    // /// Chunks are made of 32 samples (one for each LET instruction)
    // chunks: Vec<[f32; 32]>,
    /// Stack of buffers
    stack: Vec<[f32; 32]>,
    /// Store phases (one for each oscillator)
    phase: Vec<f32>,
    /// How many samples have been read from the stack
    index: usize,
}

impl<'a> Synth<'a> {
    /// Create a new synthesizer for a parameterized waveform.
    pub fn new(wave: Wave<'a>) -> Self {
        let phase = {
            let mut phase = Vec::new();
            phase.resize(wave.0.count_osc(), 0.0);
            phase
        };
        let stack = Vec::from([[0.0; 32]]);
        let index = 32;
        let wave = Some(wave);

        // panic!("{}", phase.len());

        Synth {
            wave,
            phase,
            stack,
            index,
        }
    }

    /// Return the number of configurable parameters for this waveform.
    pub fn params(&self) -> usize {
        0
    }

    /// Run synthesis with user parameters, streaming output into the provided
    /// [`Sink`]
    pub fn stream<Ch, K, const N: usize>(&mut self, mut sink: K, params: &[f32])
    where
        Ch: fon::chan::Channel + From<fon::chan::Ch32>,
        K: Sink<Ch, N>,
    {
        let sample_rate: u32 = sink.sample_rate().into();
        let synth_iter = SynthIter(self, sample_rate);

        sink.sink_with(&mut synth_iter.map(|x| x.to()));
    }

    /// Synthesis
    fn synthesize(&mut self, sample_rate: u32) -> f32 {
        if self.index < 32 {
            let sample = self.stack[0][self.index];
            self.index += 1;
            return sample;
        }

        let delta = (f64::from(sample_rate)).recip() as f32;
        let wave = self.wave.take().unwrap();
        self.node(&wave.0, delta, &mut 0);
        self.wave = Some(wave);

        self.index = 1;
        self.stack[0][0]
    }

    fn node(&mut self, node: &Node<'_>, delta: f32, osc: &mut usize) {
        match node {
            Node::Sig(v) => {
                self.stack.last_mut().unwrap().fill(*v);
            }
            Node::Mix(nodes) => {
                for node in *nodes {
                    self.stack.push([0.0; 32]);
                    self.node(node, delta, osc);
                    let buffer = self.stack.pop().unwrap();
                    for (out, sample) in
                        self.stack.last_mut().unwrap().iter_mut().zip(buffer)
                    {
                        *out += sample;
                    }
                }
            }
            Node::Sine { hz } => {
                let this = *osc;
                *osc += 1;
                self.node(hz, delta, osc);
                for i in self.stack.last_mut().unwrap() {
                    let hertz = *i;
                    *i = libm::cosf(self.phase[this] * core::f32::consts::TAU);
                    self.phase[this] = (self.phase[this] + delta * hertz) % 1.0;
                }
            }
            Node::Ramp { hz, curve } => {
                self.stack.push([0.0; 32]);
                self.node(curve, delta, osc);
                let curve = self.stack.pop().unwrap();

                let this = *osc;
                *osc += 1;
                self.node(hz, delta, osc);
                for (i, curve) in
                    self.stack.last_mut().unwrap().iter_mut().zip(curve)
                {
                    let hertz = *i;
                    *i = 1.0 - (self.phase[this] * 2.0);
                    if i.is_sign_negative() {
                        let v = *i + 1.0;
                        *i = v - v * curve * (v - 1.0) - 1.0;
                    } else {
                        let w = 1.0 - *i;
                        *i = 1.0 - w + w * curve * (w - 1.0);
                    }
                    self.phase[this] = (self.phase[this] + delta * hertz) % 1.0;
                }
            }
            Node::Pulse { hz, duty, alias } => {
                self.stack.push([0.0; 32]);
                self.node(alias, delta, osc);
                let alias = self.stack.pop().unwrap();

                self.stack.push([0.0; 32]);
                self.node(duty, delta, osc);
                let duty = self.stack.pop().unwrap();

                let this = *osc;
                *osc += 1;
                self.node(hz, delta, osc);
                for ((i, alias), duty) in self
                    .stack
                    .last_mut()
                    .unwrap()
                    .iter_mut()
                    .zip(alias)
                    .zip(duty)
                {
                    let hertz = *i;
                    let phase = self.phase[this];

                    let sa = (alias * 0.5) + 0.5; // size of alias / phase
                    let sp = (duty * 0.5) + 0.5; // size of positive (+) / phase
                    let sn = 1.0 - sp; // size of negative (+) / phase
                    let dc = sp * 0.5; // center of descent alias / phase
                    let ac = 1.0 - dc; // center of ascent alias / phase
                    let lc = dc + (ac - dc) * 0.5; // center of (-1) / phase
                    let lc = lc + (sa * duty * 0.5); //
                    let pa = sp * sa; // size of positive alias / phase
                    let na = sn * sa; // size of negative alias / phase
                    let db = (sp - pa) * 0.5; // descent begin
                    let ae = 1.0 - db; // ascent end
                    let de = lc - (sn - na) * 0.5; // descent end
                    let ab = lc + (sn - na) * 0.5; // ascent begin

                    *i = if phase < db {
                        // Before descent begin
                        1.0
                    } else if phase < de {
                        // Before descent end
                        let sd = de - db; // size of descent

                        1.0 - 2.0 * (phase - db) / sd
                    } else if phase < ab {
                        // Before ascent begin
                        -1.0
                    } else if phase < ae {
                        // Before ascent end
                        let sa = ae - ab; // size of ascent

                        -1.0 + 2.0 * (phase - ab) / sa
                    } else {
                        // After ascent end, until center (+) point
                        1.0
                    };

                    self.phase[this] = (self.phase[this] + delta * hertz) % 1.0;
                }
            }
            Node::Mul(nodes) => todo!("{nodes:?}"),
            Node::Amp(main, amp) => {
                self.stack.push([0.0; 32]);
                self.node(amp, delta, osc);
                let amp = self.stack.pop().unwrap();

                *osc += 1;
                self.node(main, delta, osc);
                for (main, amp) in
                    self.stack.last_mut().unwrap().iter_mut().zip(amp.iter())
                {
                    *main *= amp;
                }
            }
            Node::File(bytes) => todo!("{:?}", bytes),
        }
    }
}

struct SynthIter<'a, 'b>(&'b mut Synth<'a>, u32);

impl Iterator for SynthIter<'_, '_> {
    type Item = fon::Frame<fon::chan::Ch32, 1>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(synth, sample_rate) = self;

        Some(synth.synthesize(*sample_rate).into())
    }
}

/// File parsing
fn parse<'a>(file: &'a [u8], inst: &'a [u8]) -> File<'a> {
    let mut inst: [u8; 4] = inst[..4].try_into().unwrap();
    let mut code = 0;

    (inst[3], code) = (code, inst[3]);

    let index: usize = u32::from_le_bytes(inst).try_into().unwrap();

    match code {
        x if x == Inst::Sig as u8 => {
            //

            File::Sig(f32::from_le_bytes(
                file[index..][..4].try_into().unwrap(),
            ))
        }
        x if x == Inst::Mix as u8 => todo!(),
        x if x == Inst::Sin as u8 => todo!(),
        x if x == Inst::Rmp as u8 => todo!(),
        x if x == Inst::Bez as u8 => todo!(),
        x if x == Inst::Sqr as u8 => todo!(),
        x if x == Inst::Pul as u8 => todo!(),
        x if x == Inst::Let as u8 => todo!(),
        x if x == Inst::Del as u8 => todo!(),
        x if x == Inst::Var as u8 => todo!(),
        x if x == Inst::Clp as u8 => todo!(),
        x if x == Inst::Mul as u8 => todo!(),
        x if x == Inst::Amp as u8 => todo!(),
        x if x == Inst::Wht as u8 => todo!(),
        x if x == Inst::Pnk as u8 => todo!(),
        x if x == Inst::Min as u8 => todo!(),
        x if x == Inst::Max as u8 => todo!(),
        _ => panic!("Invalid instruction"),
    }
}

/// Append synth output to a `Vec`.
fn save(node: Node<'_>, out: &mut Vec<u8>) {
    // Magic number
    out.extend(b"\xFF\xFETwAnG\0");

    match node {
        Node::Sig(v) => {
            let bytes = v.to_le_bytes();
            let index = u32::try_from(out.len()).unwrap().to_le_bytes();

            out.extend(bytes);
        }
        Node::Mix(nodes) => todo!("{:?}", nodes),
        Node::Sine { hz } => todo!("{:?}", hz),
        Node::Ramp { hz, curve } => todo!("{:?} {:?}", hz, curve),
        Node::Pulse { hz, duty, alias } => {
            todo!("{:?} {:?} {:?}", hz, duty, alias)
        }
        Node::Mul(nodes) => todo!("{nodes:?}"),
        Node::Amp(main, amp) => todo!("{main:?} {amp:?}"),
        Node::File(bytes) => todo!("{:?}", bytes),
    }
}
