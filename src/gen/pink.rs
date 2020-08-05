use super::Generator;
use core::time::Duration;

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

fn pfir(pfirm: [f64; 6]) -> [i32; 64] {
    let mut pfir = [0; 64];
    for (i, v) in pfir.iter_mut().enumerate() {
        let i = i as i32;
        let a = i / 8;
        let a = a * 8;
        let b = match i % 8 {
            0 => [0, 1, 2, 3, 4, 5],
            b => [b; 6],
        };
        *v = (pfirm[0] * (2 * (a >> b[0] & 1) - 1) as f64
            + pfirm[1] * (2 * (a >> b[1] & 1) - 1) as f64
            + pfirm[2] * (2 * (a >> b[2] & 1) - 1) as f64
            + pfirm[3] * (2 * (a >> b[3] & 1) - 1) as f64
            + pfirm[4] * (2 * (a >> b[4] & 1) - 1) as f64
            + pfirm[5] * (2 * (a >> b[5] & 1) - 1) as f64) as i32
    }
    pfir
}

/// Pink Noise Generator using algorithm described in research paper
/// [A New Shade of Pink](https://github.com/Stenzel/newshadeofpink/blob/master/newshadeofpink.pdf).
#[derive(Clone)]
#[allow(missing_copy_implementations)]
pub struct Pink {
    pfira: [i32; 64],
    pfirb: [i32; 64],
    lfsr: i32,
    inc: i32,
    dec: i32,
    accu: i32,
    pncnt: u8,
    which: u8,
    bit: i32,
}

impl std::fmt::Debug for Pink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pink")
    }
}

impl Default for Pink {
    fn default() -> Self {
        Self::new()
    }
}

impl Pink {
    /// Create a new Pink Noise Sampler.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            pfira: pfir([
                2048.0 * 1.190566,
                2048.0 * 0.162580,
                2048.0 * 0.002208,
                2048.0 * 0.025475,
                2048.0 * -0.001522,
                2048.0 * 0.007322,
            ]),
            pfirb: pfir([
                2048.0 * 0.001774,
                2048.0 * 0.004529,
                2048.0 * -0.001561,
                2048.0 * 0.000776,
                2048.0 * -0.000486,
                2048.0 * 0.002017,
            ]),
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
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x800i32;
        self.inc ^= self.bit & 0x800i32;
        self.b()
    }

    fn b(&mut self) -> i16 {
        self.accu += self.inc - self.dec;
        self.lfsr ^= self.bit & 0x46000001i32;
        (self.accu
            + self.pfira[(self.lfsr & 0x3fi32) as usize]
            + self.pfirb[(self.lfsr >> 6i32 & 0x3fi32) as usize]) as i16
    }

    fn c(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x400i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x400i32;
        self.inc ^= self.bit & 0x400i32;
        self.b()
    }

    fn d(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x200i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x200i32;
        self.inc ^= self.bit & 0x200i32;
        self.b()
    }

    fn e(&mut self) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !0x100i32;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & 0x100i32;
        self.inc ^= self.bit & 0x100i32;
        self.b()
    }

    fn f(&mut self, mask: i32) -> i16 {
        self.bit = self.lfsr >> 31i32;
        self.dec &= !mask;
        self.lfsr <<= 1i32;
        self.dec |= self.inc & mask;
        self.inc ^= self.bit & mask;
        self.b()
    }
}

impl Generator for Pink {
    fn sample(&mut self, _duration: Duration) -> f64 {
        // Different functions for each sample.
        let r = match self.which {
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
        } as f64
            / (std::i16::MAX as f64);
        self.which += 1;
        self.which %= 16;
        r
    }
}
