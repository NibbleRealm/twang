# Twang
Library for pure Rust advanced audio synthesis.

## Goals
- Auto-vectorized audio synthesis.
- Pure Rust

Links to examples for each will be provided as they are implemented.
- [Additive synthesis](https://github.com/AldaronLau/twang/blob/master/examples/piano.rs)
  (Compositing sounds, usually sine waves - used in analysis / resynthesis along
  with FFT)
- Subtractive synthesis (Shaping sawtooth wave, which contains both odd and even
  frequencies, with filters to boost or reduce frequencies)
- Frequency Modulation synthesis (Modulating *frequency* with a waveform)
  - Phase Modulation (PM) synthesis - An implementation of FM used in popular
    1980s synthesizers
- Phase Distortion synthesis (Inverted **Frequency Counter** multiplied by sine
  wave at resonance frequency: A higher frequency counter that starts over /
  resets at the end of the fundamental frequency)
- Physical modeling synthesis (Karplus-Strong algorithm, other Digital waveguide
  synthesis using d'Alembert's algorithm)
- Sample-Based / Wavetable Lookup synthesis (Whats used in MIDI)
- Linear arithmetic synthesis (PCM sampled attack + subtractive synthesis
  sustain)
- Vector synthesis (Mix of four sounds based on an X-Y plane)
- Phase Offset Modulation Synthesis (two instances of a periodic waveform kept
  slightly out of sync with each other, then are either multiplied or
  subtracted)
- Arbitrary waveform synthesis (Defined samples occuring at different times in
  a waveform, called waypoints - either jump or interpolate {video game music})

### Waveforms
Oscillators:
- [Frequency Counter (Fc / Sawtooth)](https://docs.rs/twang/0.3.0/twang/struct.Fc.html) -
  odd and even harmonics that decrease at -6 dB/octave
- [Pulse Wave](https://docs.rs/twang/0.3.0/twang/sig/struct.Signal.html#method.pulse)
  (When ½ duty cycle set to 1.0 is a Square Wave - odd harmonics that decrease
  at -6 dB/octave)
- [Sine Wave](https://docs.rs/twang/0.3.0/twang/sig/struct.Signal.html#method.sine) -
  no harmonics
- Triangle Wave - odd harmonics that decrease at -12 dB/octave

"Voltage" Controlled filter:
- Lowpass (Most Common) / Highpass
- Bandpass / Notch

Envelope (example: ADSR):
- Press: Attack (time, oscillator:Fc), Hold?, Decay (time, oscillator:Fc.inv), …
- Hold: Sustain (oscillator:Pulse(1.0).Gain(level))
- Release: Release (time, oscillator:Fc.inv)

"Voltage" Controlled amplifier (multiplication)

## Effects
### Gated Reverb
Reverb without reflections.

 1. Generate Percussive Sound
 2. Add Reverb
 3. Compress (optional)
 4. Apply Noise Gate To Resulting Sound (Side chaining the original sound)
 5. About 1/2 second hold, few millisecond release time envelope on Noise Gate
 6. Mix with original sound

### Reverb And Echo
Reverb is just echo that takes place in less than 50 milliseconds, so you can
use the same APIs.

## Decibel Normalization
When generating waveforms you may want to normalize by the volume of the wave,
which the amplitude doesn't always accurately approximate.  First, calculate the
RMS (Root Mean Square).
```
let sum = 0
for sample in samples {
    sum += sample * sample
}
sum /= samples.len()
let rms = sum.sqrt()
```

Now the peak value:
```
let mut peak_amplitude = 0
for sample in samples {
    peak_amplitude = peak_amplitude.max(sample.abs())
}
```

Next the crest factor:
```
let crest_factor = peak_amplitude / rms
```

Then decibels:

```
let papr = 20 * log10(crest_factor)
```

Crest Factor Post-processed Recording 4–8 / 12–18 dB headroom
Crest Factor Unprocessed Recording 8–10 / 18–20 dB headroom

And actual perceived volume should be calculated with
[ITU-R BS.1770-4](https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1770-4-201510-I!!PDF-E.pdf)

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
