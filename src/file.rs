// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

//! Twang synthesis file format

use alloc::vec::Vec;
use fon::{Audio, Sink};
use fon::chan::{Channel, Ch32};

/*
/// A twang synthesis operation
enum Op {
    /// Pop last value on audio stack
    Pop,
    /// Push value onto audio stack
    Psh(Ch32),
    /// Swap by index 
    Swp(u32),
    /// Swap and pop by index
    Sap(u32),
    /// In-place oscillator transformation (in/out: frequency)
    Osc(Ch32),
    /// In-place bezier transformation (in/out: source, in:curve)
    Bez,
}

/// A synthesizer
pub struct Synth {
    // Audio stack
    stack: Vec<Chunk>,
    // Synthesis program
    ops: Vec<Op>,
}

impl Synth {
    /// Stream synthesized samples into a sink
    pub fn stream(&mut self) {
        const ONE: Ch32 = Ch32::new(1.0);

        let step = (48_000.0f32).recip();

        for op in self.ops.iter_mut() {
            match op {
                Op::Pop => { self.stack.pop(); }
                Op::Psh(ref chan) => self.stack.push(Chunk([*chan; 32])),
                Op::Swp(idx) => {
                    let tmp = self.stack.swap_remove(*idx as usize);
                    self.stack.push(tmp)
                },
                Op::Sap(idx) => { self.stack.swap_remove(*idx as usize); },
                Op::Osc(ref mut time) => {
                    let len = self.stack.len();
                    let a = self.stack.get_mut(len - 1).unwrap();
                    for rate in a.0.iter_mut() {
                        let delta = *rate * step;
                        *rate = *time;
                        *time += delta;
                    }
                }
                Op::Bez => {
                    let len = self.stack.len();
                    for i in 0..32 {
                        let curve = self.stack[len - 2].0[i];
                        let src = &mut self.stack[len - 1].0[i];
                        if src.to_f32().is_sign_negative() {
                            let v = *src + ONE;
                            *src = v - v * curve * (v - ONE) - ONE;
                        } else {
                            let w = ONE - *src;
                            *src = ONE - w + w * curve * (w - ONE);
                        }
                    }
                }
            }
        }
    }
}
*/

/// Node in the synthesis tree
#[derive(Debug)]
enum Node {
    Source(Chunk),
    Line(Value),

    WaveT(Table, Const),
    WaveC(Table, Chunk),
    WaveV(Table, Value),
    
    WaypointT(Table, Const),
    WaypointC(Table, Chunk),
    WaypointV(Table, Value),

    BezierTT(Const, Const),
    BezierTC(Const, Chunk),
    BezierTV(Const, Value),
    BezierCT(Chunk, Const),
    BezierCC(Chunk, Chunk),
    BezierCV(Chunk, Value),
    BezierVT(Value, Const),
    BezierVC(Value, Chunk),
    BezierVV(Value, Value),


    /// Frequency Counter
    ///
    /// A frequency counter is a sawtooth wave.
    Freq(Ch32, u32),
    /// Trapazoid wave
    ///
    /// Subtree params: fc, rise, hold, fall.
    Zoid(u32, u32, u32, u32),
}

/// Sample input
///
/// An input is 1 value.
#[derive(Copy, Clone, Debug)]
pub struct Value(pub u32);

/// Sample chunk input / cache
///
/// A chunk contains 32 samples.
#[derive(Copy, Clone, Debug)]
pub struct Chunk(pub u32);

/// Sample wavetable/waypoint input
///
/// A wavetable contains any number of samples.
#[derive(Copy, Clone, Debug)]
pub struct Table(pub u32);

/// Sample constant
#[derive(Copy, Clone, Debug)]
pub struct Const(pub Ch32);

impl Sampler for Value {
    fn to_any(self) -> Any {
        Any::Value(self)
    }
}

impl Sampler for Chunk {
    fn to_any(self) -> Any {
        Any::Chunk(self)
    }
}

impl Sampler for Const {
    fn to_any(self) -> Any {
        Any::Const(self)
    }
}

mod seal {
    use super::*;

    pub trait Sampler {
        fn to_any(self) -> Any;
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Any {
        Value(Value),
        Chunk(Chunk),
        Const(Const),
    }
}

use self::seal::{Any, Sampler};


/// Builder for a synth
///
/// Inputs -> Program -> Output
#[derive(Debug)]
pub struct SynthBuilder {
    nodes: Vec<Node>,
    input_samples: Vec<f32>,
    input_buffers: Vec<[Ch32; 32]>,
    input_wtables: Vec<Vec<Ch32>>,
}

impl SynthBuilder {
    /// Add chunked audio from an external source
    pub fn mix_source(mut self, chunk: Chunk) -> Self {
        self.input_buffers.resize((chunk.0 + 1).try_into().unwrap(), [Ch32::default(); 32]);
        self.nodes.push(Node::Source(chunk));
        self
    }

    /// Add line wave
    ///
    /// A line wave is a horizontal line, silence to human ears.
    pub fn mix_line(mut self, value: Value) -> Self {
        self.input_samples.resize((value.0 + 1).try_into().unwrap(), 0.0);
        self.nodes.push(Node::Line(value));
        self
    }
    
    /// Add wavetable
    ///
    /// A wave table is a collection of samples that are slowed down or sped up
    /// to make the pitch higher or lower.
    pub fn mix_wave(mut self, table: Table, freq: impl Sampler) -> Self {
        self.input_wtables.resize((table.0 + 1).try_into().unwrap(), Vec::new());
        self.nodes.push(match freq.to_any() {
            Any::Value(x) => Node::WaveV(table, x),
            Any::Chunk(x) => Node::WaveC(table, x),
            Any::Const(x) => Node::WaveT(table, x),
        });
        self
    }

    /// Add waypoint input
    ///
    /// A ways table is almost the same thing as a wavetable, except allows
    /// aliasing.
    pub fn mix_ways(mut self, table: Table, freq: impl Sampler) -> Self {
        self.input_wtables.resize((table.0 + 1).try_into().unwrap(), Vec::new());
        self.nodes.push(match freq.to_any() {
            Any::Value(x) => Node::WaypointV(table, x),
            Any::Chunk(x) => Node::WaypointC(table, x),
            Any::Const(x) => Node::WaypointT(table, x),
        });
        self
    }

    /// Bezier wave
    ///
    /// A bezier wave is a waveform formed by two symmetrical bezier curves.
    pub fn bezier(mut self, fc: impl Sampler, speed: impl Sampler) -> Self {
        self.nodes.push(match fc.to_any() {
            Any::Value(x) => match speed.to_any() {
                Any::Value(y) => Node::BezierVV(x, y),
                Any::Chunk(y) => Node::BezierVC(x, y),
                Any::Const(y) => Node::BezierVT(x, y),
            },
            Any::Chunk(x) => match speed.to_any() {
                Any::Value(y) => Node::BezierCV(x, y),
                Any::Chunk(y) => Node::BezierCC(x, y),
                Any::Const(y) => Node::BezierCT(x, y),
            },
            Any::Const(x) => match speed.to_any() {
                Any::Value(y) => Node::BezierTV(x, y),
                Any::Chunk(y) => Node::BezierTC(x, y),
                Any::Const(y) => Node::BezierTT(x, y),
            },
        });
        self
    }
}




/*
#[derive(Debug)]
struct EnvelopeComponent {
    time: i32,
    gain: i32,
}

#[derive(Debug)]
enum Function {
    Sine {
        hz: i32,
        duty: Option<i32>,
        zero: Option<i32>,
        peak: Option<i32>,
    },
    White {
        seed: Option<u32>,
    },
    Pink {
        seed: Option<u32>,
    },
    Phase {
        func: i32,
        offset: i32,
    },
    Line(f32),
    Mix {
        func: i32,
        amt: i32,
    },
    Limit {
        func: i32,
        ceil: Option<i32>,
        ratio: Option<i32>,
        knee: Option<i32>,
    },
    Clamp {
        func: i32,
        min: Option<i32>,
        max: Option<i32>,
        shift: Option<i32>,
    },
    Envelope {
        func: i32,
        with: Vec<EnvelopeComponent>,
    },
    Reverb,
    Shape,
    Table,
}

#[derive(Debug)]
struct Sampler {
    // Input samplers
    args: i32,
    func: Function,
}

/// A Twang Synthesizer Instance
#[derive(Debug)]
pub struct Synth {
    def: Vec<Sampler>,
    synth: i32,
    buffer: Vec<Ch32>,
}

impl Synth {
    /// Get a builder to construct this synth
    pub fn builder() -> SynthBuilder {
        SynthBuilder::new()
    }

    /// Stream synthesized samples into a [`Sink`].
    pub fn stream(&mut self, sink: impl Sink<Ch32, 1>) {
        sink.sink_with(core::iter::from_fn(|| {
            if self.buffer.is_empty() {
                self.buffer.pop()
            } else {
                self.buffer.pop()
            }
        }));
    }

    /// Apply a function to a chunk of audio.
    fn apply(&self, chunk: &mut [Ch32; 32], f: Function) {
        use Function::*;
        match f {
            Zoid {
                hz,
                rise,
                hold,
                fall,
            } => {
                todo!()
            },
            Sine {
                hz,
                duty,
                zero,
                peak,
            } => {
                todo!()
            },
            White {
                seed,
            } => {
                todo!()
            },
            Pink {
                seed,
            } => {
                todo!()
            },
            Phase {
                func,
                offset,
            } => {
                todo!()
            },
            Line(value) => {
                for sample in chunk.iter_mut() {
                    *sample = value.into();
                }
            },
            Mix {
                func,
                amt,
            } => {
                todo!()
            },
            Limit {
                func,
                ceil,
                ratio,
                knee,
            } => {
                todo!()
            },
            Clamp {
                func,
                min,
                max,
                shift,
            } => {
                todo!()
            },
            Envelope {
                func,
                with,
            } => {
                todo!()
            },
            Reverb => todo!(),
            Shape => todo!(),
            Table => todo!(),
        }
    }
}

/// Builder for [`Synth`]
#[derive(Debug)]
pub struct SynthBuilder {
    synth: Synth,
}

impl SynthBuilder {
    fn new() -> Self {
        let synth = Synth {
            def: Vec::new(),
            synth: i32::MIN,
            buffer: [Ch32::new(0.0); 32],
        };

        Self { synth }
    }

    /// Create a line wave (constant, makes no sound)
    pub fn line(mut self, value: impl Into<Ch32>) -> Self {
        self.synth.def.push(Function::Line(value.into().to_f32()));
        self
    }


}
*/
