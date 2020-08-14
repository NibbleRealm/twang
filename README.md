# Twang
Library for pure Rust advanced audio synthesis.

## Goals
- Auto-vectorized audio synthesis.
- Pure Rust

Links to examples for each will be provided as they are implemented.
- [Additive synthesis](https://github.com/AldaronLau/twang/blob/master/examples/additive.rs)
  (Compositing sounds, usually sine waves - used in analysis / resynthesis along
  with FFT)
- Subtractive synthesis (Oscillators to generate waves, then shape with filters
  to boost or reduce frequencies)
- Frequency Modulation synthesis (Modulating *frequency* with a waveform)
- Phase Distortion synthesis (Inverted **Frequency Counter** multiplied by sine
  wave at resonance frequency: A higher frequency counter that starts over /
  resets at the end of the fundamental frequency)
- Physical modeling syntheis (Karplus-Strong algorithm, other Digital waveguide
  synthesis using d'Alembert's algorithm)
- Sample-Based / Wavetable Lookup synthesis (Whats used in MIDI)
- Linear arithmetic synthesis (PCM sampled attack + subtractive synthesis
  sustain)
- Vector synthesis (Mix of four sounds based on an X-Y plane)

### Waveforms
Oscillators:
- [Frequency Counter (Fc / Sawtooth)](https://docs.rs/twang/0.3.0/twang/struct.Fc.html)
- Pulse Wave (When duty cycle set to 0.5 is a Square Wave, Sawtooth wave minus
  phase shifted version of itself)
- Sine Wave

"Voltage" Controlled filter:
- Lowpass (Most Common) / Highpass
- Bandpass / Notch

Envelope (ADSR example):
- Press: Attack (time, oscillator:Fc), Hold?, Decay (time, oscillator:Fc.inv), …
- Hold: Sustain (oscillator:Pulse(1.0).Gain(level))
- Release: Release (time, oscillator:Fc.inv)

"Voltage" Controlled amplifier (multiplication)

## Getting Started
Examples can be found in the [Documentation](https://docs.rs/twang) and the
examples folder.

## License
The `twang` crate is distributed under any of

- The terms of the
  [MIT License](https://github.com/AldaronLau/twang/blob/master/LICENSE-MIT)
- The terms of the
  [Apache License (Version 2.0)](https://github.com/AldaronLau/twang/blob/master/LICENSE-APACHE)
- The terms of the
  [Zlib License](https://github.com/AldaronLau/twang/blob/master/LICENSE-ZLIB)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.
