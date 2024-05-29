# Pulse Oscillators

A true pulse oscillator only contains the samples -1 and 1.  The `alias` amount
is used to determine how close it should be to a sawtooth wave.

## Property Parameters

### `alias`

Alias controls the distortion level by dividing the amplitude.

 - -1: downwards sloping sawtooth wave
 - 0: symmetrical square wave
 - 1: upwards sloping sawtooth wave

## Alternate Interpolation

Send a triangle wave as input into a pulse oscillator to interpolate between
a triangle and pulse wave.

## Sound Theory

The square wave, similar to the triangle wave only has odd harmonics.
