# Additive Synthesis
Additive synthesis is probably the simplest synthesis algorithm to understand.
The fundamental idea is that all sounds are made up of an infinite number of
sine waves at different volumes mixed together.  Of course, when synthesizing
you can only mix a finite amount of sine waves, otherwise you would be able to
synthesize any sound with additive synthesis!  That said, you can approximate
any sound with additive synthesis, even to the point where it's
indistiguishable!

But, additive synthesis isn't limited to just sine waves; You can use other
kinds of waveforms as well.  Interestingly, additive synthesis is actually the
exact same thing as mixing!  This means when you mix the two waveforms together,
you are able to hear both of them playing together.  Of course, the more
waveforms you add together, the more difficult it becomes to pick them out, and
the less it sounds like multiple "instruments" or "voices".

## Fast Fourier Tranform
TODO

# (Electric) Piano Example
TODO

[Example Code](https://github.com/AldaronLau/twang/blob/stable/examples/piano.rs)
