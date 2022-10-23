# Pulse Oscillators
A pulse oscillator is the simplest waveform, because it only contains the
samples -1 (off) and 1 (on).  There are two parameters for changing the shape of
the waveform; First, there is `duty` which refers to the duty cycle - when set
to 0 produces a square wave (50% on/off; -1 is 100% off, 1 is 100% on).  Then,
there is `alias` which introduces aliasing.  The aliasing implemented in twang
is linear interpolation between the samples.  Set to -1 for rectangle, 0 for
trapazoid, and 1 for triangle wave.
