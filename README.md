# Twang
Library for pure Rust advanced audio synthesis.

## Goals
- Fast: Auto-vectorized audio synthesis.
- Pure Rust: No system dependencies outside std.

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
- [Frequency Counter (Fc / Sawtooth)](https://docs.rs/twang/latest/twang/struct.Fc.html) -
  odd and even harmonics that decrease at -6 dB/octave
- [Pulse Wave](https://docs.rs/twang/latest/twang/sig/struct.Signal.html#method.pulse)
  (When ½ duty cycle set to 1.0 is a Square Wave - odd harmonics that decrease
  at -6 dB/octave)
- [Sine Wave](https://docs.rs/twang/latest/twang/sig/struct.Signal.html#method.sine) -
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

[Gated Reverb](https://github.com/AldaronLau/twang/blob/master/examples/gate.rs)

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
Licensed under either of
 - Apache License, Version 2.0
   ([LICENSE_APACHE_2_0.txt](https://github.com/AldaronLau/twang/blob/main/LICENSE_APACHE_2_0.txt) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
 - Boost License, Version 1.0
   ([LICENSE_BOOST_1_0.txt](https://github.com/AldaronLau/twang/blob/main/LICENSE_BOOST_1_0.txt) or
   [https://www.boost.org/LICENSE_1_0.txt](https://www.boost.org/LICENSE_1_0.txt))

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Anyone is more than welcome to contribute!  Don't be shy about getting involved,
whether with a question, idea, bug report, bug fix, feature request, feature
implementation, or other enhancement.  Other projects have strict contributing
guidelines, but this project accepts any and all formats for pull requests and
issues.  For ongoing code contributions, if you wish to ensure your code is
used, open a draft PR so that I know not to write the same code.  If a feature
needs to be bumped in importance, I may merge an unfinished draft PR into it's
own branch and finish it (after a week's deadline for the person who openned
it).  Contributors will always be notified in this situation, and given a choice
to merge early.

All pull request contributors will have their username added in the contributors
section of the release notes of the next version after the merge, with a message
thanking them.  I always make time to fix bugs, so usually a patched version of
the library will be out a few days after a report.  Features requests will not
complete as fast.  If you have any questions, design critques, or want me to
find you something to work on based on your skill level, you can email me at
[jeronlau@plopgrizzly.com](mailto:jeronlau@plopgrizzly.com).  Otherwise,
[here's a link to the issues on GitHub](https://github.com/AldaronLau/twang/issues),
and, as always, make sure to read and follow the
[Code of Conduct](https://github.com/AldaronLau/twang/blob/main/CODE_OF_CONDUCT.md).
