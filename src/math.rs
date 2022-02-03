// Copyright © 2018-2022 The Twang Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use core::ops::Rem;

/// Floating point methods currently only available on std, that may be
/// implemented with the libm crate as dependency of core in the future.
pub(crate) trait Libm: Rem<Output = Self> + Sized {
    fn cos(self) -> Self;
    fn abs(self) -> Self;
    fn copysign(self, other: Self) -> Self;
    fn signum(self) -> Self;
    fn exp(self) -> Self;
}

impl Libm for f32 {
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cosf(self)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    #[inline(always)]
    fn copysign(self, other: Self) -> Self {
        libm::copysignf(self, other)
    }

    #[inline(always)]
    fn signum(self) -> Self {
        if self.is_nan() {
            Self::NAN
        } else {
            1.0_f32.copysign(self)
        }
    }

    #[inline(always)]
    fn exp(self) -> Self {
        libm::expf(self)
    }
}

#[cfg(test)]
mod tests {
    // Tests stolen from https://doc.rust-lang.org/src/std/f32.rs.html

    #[test]
    fn math_signum() {
        let f = 3.5_f32;

        assert_eq!(f.copysign(0.42), 3.5_f32);
        assert_eq!(f.copysign(-0.42), -3.5_f32);
        assert_eq!((-f).copysign(0.42), 3.5_f32);
        assert_eq!((-f).copysign(-0.42), -3.5_f32);

        assert!(f32::NAN.copysign(1.0).is_nan());
    }

    #[test]
    fn math_exp() {
        let one = 1.0f32;
        // e^1
        let e = one.exp();

        // ln(e) - 1 == 0
        let abs_difference = (e.ln() - 1.0).abs();

        assert!(abs_difference <= f32::EPSILON);
    }
}
