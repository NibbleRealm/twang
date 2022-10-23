# Angular Modulation Synthesis
Angular modulation includes both frequency modulation and phase modulation
synthesis.  The two are very similar methods, and essentially do the same thing.
The only difference is in the implementation; frequency modulation modulates the
velocity of the waveform, while phase modulation modulates the offset position.

Frequency modulation synthesis is when the frequency variable of an oscillator
is set to another waveform.

Phase modulation synthesis is when the phase variable of an oscillator is set to
another waveform.  Phase modulation synthesis was popular in 1980s synthesizers.

## Phase Distortion
Phase distortion builds upon angular modulation synthesis.
 1. Do angular modulation of a waveform
 2. After each cycle of the lower frequency, reset the high frequency oscillator
 3. Multiply by decreasing sawtooth wave at the lower frequency to hide the
    "jump"

## Waveshaping (Direct Phase Modulation)
Waveshaping builds upon phase modulation synthesis.  In this type of modulation,
the two oscillators used must have the same frequency.

## Phase Limiting
Phase limiting is an improvement on the phase distortion synthesis "invented" by
Casio (which as far as I know, was "invented" by me - Jeron Aldaron Lau).  The
idea is simple; take the first two steps of phase distortion synthesis as they
are, then instead of multiplying by a decreasing sawtooth oscillator, multiply
by a decreasing bezier oscillator.  This reduces the amount of lost amplitude
during phase distortion synthesis.  Additionally, splice a plateau at the
beginning of the bezier waveform to only diminish the amplitude after the phase
has passed a specified point.
