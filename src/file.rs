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
use alloc::string::String;
use hashbrown::hash_map::HashMap;

#[derive(Debug)]
struct FileEnvelopeComponent {
    time: String,
    gain: String,
}

#[derive(Debug)]
struct EnvelopeComponent {
    time: i32,
    gain: i32,
}

#[derive(Debug)]
enum FileFunction {
    Zoid {
        hz: String,
        rise: String,
        hold: String,
        fall: String,
    },
    Sine {
        hz: String,
        duty: Option<String>,
        zero: Option<String>,
        peak: Option<String>,
    },
    White {
        seed: Option<u32>,
    },
    Pink {
        seed: Option<u32>,
    },
    Phase {
        func: String,
        offset: String,
    },
    Line(Option<f32>),
    Mix {
        func: String,
        amt: String,
    },
    Limit {
        func: String,
        ceil: Option<String>,
        ratio: Option<String>,
        knee: Option<String>,
    },
    Clamp {
        func: String,
        min: Option<String>,
        max: Option<String>,
        shift: Option<String>,
    },
    Envelope {
        func: String,
        with: Vec<FileEnvelopeComponent>,
    },
    Reverb,
    Shape,
    Table,
}

#[derive(Debug)]
enum Function {
    Zoid {
        hz: i32,
        rise: i32,
        hold: i32,
        fall: i32,
    },
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
    Line(Option<f32>),
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
struct FileSampler {
    // Input samplers
    args: Vec<String>,
    func: FileFunction,
}

#[derive(Debug)]
struct Sampler {
    // Input samplers
    args: i32,
    func: Function,
}

/// A synthesis file
#[derive(Debug)]
pub struct File {
    def: HashMap<String, FileSampler>,
    synth: String,
}

/// Compiled synthesis
#[derive(Debug)]
pub struct Twang {
    def: Vec<Sampler>,
    synth: i32,
}
