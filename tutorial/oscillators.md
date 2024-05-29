# Oscillators

Oscillators are at the core of audio synthesis.  The basic concept is that a
sound is composed of a repeating pattern.  How many times this pattern repeats
is the frequency of the sound.  The higher the frequency (more times per second)
the higher the pitch.  The repeating pattern is often called a "waveform".

## Sawtooth Wave

The sawtooth wave is the simplest waveform to generate, as it's just the
identity function; so as the phase goes from -1 to 1, the amplitude goes from -1
to 1.  It is the waveform that is returned from the _**phase accumulator**_ (a
variable that determines the phase from a frequency).
