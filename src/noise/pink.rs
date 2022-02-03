// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use fon::chan::Ch16;

// Constants PFIRA and PFIRB
include!(concat!(env!("OUT_DIR"), "/pink.rs"));

#[inline(always)]
fn pnmask(pncnt: u8) -> u8 {
    match pncnt % 16 {
        _x if _x % 2 != 0 => 0x80,
        _x if _x % 4 != 0 => 0x40,
        _x if _x % 8 != 0 => 0x20,
        8 => 0x10, // _x if _x % 16 != 0
        _ => match pncnt / 16 {
            // only 0
            _x if _x % 2 != 0 => 8,
            _x if _x % 4 != 0 => 4,
            _x if _x % 8 != 0 => 2,
            8 => 1, // _x if _x % 16 != 0
            _ => 0, // only 0
        },
    }
}

/// Pink Noise Generator using algorithm described in research paper
/// [A New Shade of Pink](https://github.com/Stenzel/newshadeofpink/blob/master/newshadeofpink.pdf).
#[derive(Debug, Copy, Clone)]
pub struct Pink {
    lfsr: i32,
    inc: i32,
    dec: i32,
    accu: i32,
    pncnt: u8,
    which: u8,
    bit: i32,
}

impl Default for Pink {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl Pink {
    /// Create a new Pink Noise Sampler.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            lfsr: 0x5eed41f5i32,
            inc: 0xccc,
            dec: 0xccc,
            accu: 0,
            pncnt: 0,
            which: 0,
            bit: 0,
        }
    }

    fn a(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x800i32;
        self.lfsr <<= 1;
        self.dec |= self.inc & 0x800i32;
        self.inc ^= self.bit & 0x800i32;
        self.b()
    }

    fn b(&mut self) -> i16 {
        self.accu += self.inc - self.dec;
        self.lfsr ^= self.bit & 0x46000001i32;
        (self.accu
            + PFIRA[(self.lfsr & 0x3fi32) as usize]
            + PFIRB[(self.lfsr >> 6i32 & 0x3fi32) as usize]) as i16
    }

    fn c(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x400i32;
        self.lfsr <<= 1;
        self.dec |= self.inc & 0x400i32;
        self.inc ^= self.bit & 0x400i32;
        self.b()
    }

    fn d(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x200i32;
        self.lfsr <<= 1;
        self.dec |= self.inc & 0x200i32;
        self.inc ^= self.bit & 0x200i32;
        self.b()
    }

    fn e(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x100i32;
        self.lfsr <<= 1;
        self.dec |= self.inc & 0x100i32;
        self.inc ^= self.bit & 0x100i32;
        self.b()
    }

    fn f(&mut self, mask: i32) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !mask;
        self.lfsr <<= 1;
        self.dec |= self.inc & mask;
        self.inc ^= self.bit & mask;
        self.b()
    }

    /// Get next sample from the noise generator.
    #[inline(always)]
    pub fn step(&mut self) -> fon::chan::Ch32 {
        // Different functions for each sample.
        let pink = match self.which {
            _x if _x % 2 != 0 => self.a(), // odd #s
            _x if _x % 4 != 0 => self.c(),
            _x if _x % 8 != 0 => self.d(),
            0 => {
                let mask = pnmask(self.pncnt).into();
                self.pncnt = self.pncnt.wrapping_add(1);
                self.f(mask)
            }
            8 => self.e(),
            _ => unreachable!(),
        };
        self.which += 1;
        self.which %= 16;

        Ch16::new(pink).into()
    }
}
