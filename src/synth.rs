// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use fon::chan::{Channel, Ch32};
use fon::{Audio, Frame, Stream};
use std::fmt::{Debug, Error, Formatter};

/// A synthesizer stream.
pub struct Synth<S, const CH: usize>(
    S,
    Box<dyn FnMut(&mut S, Frame<Ch32, CH>) -> Frame<Ch32, CH>>,
);

impl<S, const CH: usize> Debug for Synth<S, CH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Synth")
    }
}

impl<S, const CH: usize> Synth<S, CH> {
    /// Create a new synthesizer function.
    pub fn new<F>(s: S, f: F) -> Self
    where
        F: 'static + FnMut(&mut S, Frame<Ch32, CH>) -> Frame<Ch32, CH>,
    {
        Self(s, Box::new(f))
    }
}

impl<S, const CH: usize> Stream<Ch32, CH> for Synth<S, CH> {
    fn sample_rate(&self) -> u32 {
        48_000
    }

    #[inline(always)]
    fn sink<C: Channel, const N: usize>(&mut self, buf: &mut Audio<C, N>)
    where
        C: From<Ch32>,
    {
        assert_eq!(48_000, buf.sample_rate());

        for out in buf.iter_mut() {
            *out = self.1(&mut self.0, Default::default()).to();
        }
    }
}
