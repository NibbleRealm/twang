# Introduction
Twang is a Rust library for doing audio synthesis.  The API of Twang uses a
dependency tree (type of DAG or directed acyclic graph) of audio nodes rather
than supporting arbitrary graphs.  The tree makes it easier to visualize what's
happening during synthesis.  The root node of the tree is the final operation
that produces the synthesized sound.

This book will teach not only how to use Twang, but also about how all the
different methods of audio synthesis work.  Each chapter will cover a synthesis
method and provide an example of how to use it with Twang.  You will find that
the reason there are so many methods of synthesis is because they each make
their own unique type of sound from each other.  So if you want to synthesize
a specific type of sound, you choose a specific type of synthesis.  Here's a
little guide to help (you might find a way to synthesize a type of sound with a
different method than listed here, and that's ok - it's not an exact science):

 - [Additive Synthesis](additive.md): Electronic reproduction of a sound with
   reduced quality, such as an "electric piano" sound
 - [Subtractive Synthesis](subtractive.md): Electronic immitation of brass,
   saxaphone, or any other sound with rich harmonic content
 - Frequency Modulation Synthesis: Electronic immitation of bells

## Digital Audio
There's a great introduction to digital audio concepts on
[MDN](https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Audio_concepts)
that's worth checking out before reading this book.
