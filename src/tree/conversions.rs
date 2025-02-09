#[inline(always)]
fn reinterpret_signed(int: u32) -> i32 {
    i32::from_ne_bytes(int.to_ne_bytes())
}

#[inline(always)]
fn reinterpret_unsigned(int: i32) -> u32 {
    u32::from_ne_bytes(int.to_ne_bytes())
}

/// Convert non-zero [`u32`] fraction to [`f32`] (ranged 0 to 1).
#[inline(always)]
fn nonzero_u32_to_f32(fraction: u32) -> f32 {
    // Calculate leading zeros (with inferred 1)
    let leading_zeros = fraction.leading_zeros() + 1;
    // Remove leading zeros and inferred 1 to subtract from exponent
    let fraction = fraction.wrapping_shl(leading_zeros);
    // Shift right to truncate to 23-bit fraction
    let fraction = fraction >> 9;
    // Calculate -127 bias exponent
    let exponent = (127 - leading_zeros) << 23;

    // Scale up (u32 max is 2³² - 1, and we want 2³²)
    f32::from_bits(exponent | fraction)
        * f32::from_bits(0b111111100000000000000000000001)
}

/// Convert normal [`f32`] (ranged 0 to 1) to [`u32`] fraction.
#[inline(always)]
fn normal_f32_to_u32(float: f32) -> u32 {
    // Scale down (f32 max fraction is 2³², and we want 2³² - 1)
    let float =
        (float * f32::from_bits(0b111111011111111111111111111111)).to_bits();
    // Convert fraction to 23 bits
    let fraction = (float << 9) >> 1;
    // Artificially extend fraction precision, and add inferred 1
    let fraction = (1 << 31) | fraction | (fraction >> 23);
    // Extract -127 bias 8-bit negative exponent
    let exponent = reinterpret_unsigned(127 - reinterpret_signed(float >> 23));
    // Scale by exponent
    let (fraction, overflow) = fraction.overflowing_shr(exponent - 1);
    // Check if fraction should be 0 or not
    let nonzero = reinterpret_unsigned(-i32::from(!overflow));

    // Make zero if zero, otherwise no-op
    fraction & nonzero
}

/// Convert [`u32`] fraction to [`f32`] (ranged 0 to 1).
pub(crate) fn u32_to_f32(fraction: u32) -> f32 {
    // Check if fraction is 0 or not
    let nonzero = reinterpret_unsigned(-i32::from(fraction != 0));

    // Make zero if zero, otherwise no-op
    f32::from_bits(nonzero_u32_to_f32(fraction).to_bits() & nonzero)
}

/// Convert [`i32`] fraction to [`f32`] (ranged -1 to 1).
pub(crate) fn i32_to_f32(int: i32) -> f32 {
    // Split sign and magnitude from signed integer
    let sign = -i8::from(int < 0);
    let uint = int.abs_diff(sign.into());
    // Scale up unsigned integer to full range (without true zero)
    let uint = (uint * 2) + 1;

    // Copy sign back into converted float
    nonzero_u32_to_f32(uint).copysign(sign.into())
}

/// Convert [`f32`] (ranged 0 to 1) to [`u32`] fraction.
#[inline(always)]
fn f32_to_u32(float: f32) -> u32 {
    // Check if fraction is normal or not
    let normal = reinterpret_unsigned(-i32::from(float.is_normal()));
    // Flush subnormals, infinity and NaN to zero, and clamp from 0 to 1
    let float = f32::from_bits(float.to_bits() & normal).clamp(0.0, 1.0);

    // Convert to unsigned integer
    normal_f32_to_u32(float)
}

/// Convert [`f32`] (ranged -1 to 1) to [`i32`] fraction.
#[inline(always)]
fn f32_to_i32(float: f32) -> i32 {
    // Check if fraction is normal or not
    let normal = reinterpret_unsigned(-i32::from(float.is_normal()));
    // Flush subnormals, infinity and NaN to zero, and clamp from -1 to 1
    let float = f32::from_bits(float.to_bits() & normal).clamp(-1.0, 1.0);
    // Convert to unsigned integer and reduce precision
    let magnitude = reinterpret_signed(normal_f32_to_u32(float.abs()) >> 1);
    // Get offset
    let offset = -i32::from(float.is_sign_negative());
    // Get sign
    let sign = (offset * 2) + 1;

    // Construct fraction with sign, magnitude, and offset
    offset + (magnitude * sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_to_float() {
        for i in 0..10 {
            assert_eq!(
                u32_to_f32(u32::MAX / 2u32.pow(i)),
                0.5f32.powi(i.try_into().unwrap()),
            );
        }

        assert_eq!(u32_to_f32(0), 0.0);
        assert_eq!(u32_to_f32(1), 2.3283067e-10);
        assert_eq!(u32_to_f32(2), 4.6566134e-10);
        assert_eq!(u32_to_f32(3), 6.9849204e-10);
        assert_eq!(u32_to_f32(4), 9.313227e-10);
        assert_eq!(u32_to_f32(5), 1.1641533e-9);
        assert_eq!(u32_to_f32(6), 1.3969841e-9);
        assert_eq!(u32_to_f32(7), 1.6298147e-9);
        assert_eq!(u32_to_f32(8), 1.8626454e-9);
        assert_eq!(u32_to_f32(9), 2.095476e-9);
    }

    #[test]
    fn signed_to_float() {
        // Since there are more negative than positive integers, zeros are not
        // an exact match between integer and floating point
        assert_eq!(i32_to_f32(i32::MAX), 1.0);
        assert_eq!(i32_to_f32(i32::MAX / 2), 0.5);
        assert_eq!(i32_to_f32(i32::MAX / 4), 0.25);
        assert_eq!(i32_to_f32(i32::MAX / 8), 0.125);
        assert_eq!(i32_to_f32(i32::MAX / 16), 0.0625);
        assert_eq!(i32_to_f32(i32::MAX / 32), 0.03125);
        assert_eq!(i32_to_f32(i32::MAX / 64), 0.015625);
        assert_eq!(i32_to_f32(i32::MAX / 128), 0.0078125);
        assert_eq!(i32_to_f32(i32::MAX / 256), 0.00390625);
        assert_eq!(i32_to_f32(i32::MAX / 512), 0.001953125);
        assert_eq!(i32_to_f32(7), 3.49246e-9);
        assert_eq!(i32_to_f32(6), 3.0267988e-9);
        assert_eq!(i32_to_f32(5), 2.5611373e-9);
        assert_eq!(i32_to_f32(4), 2.095476e-9);
        assert_eq!(i32_to_f32(3), 1.6298147e-9);
        assert_eq!(i32_to_f32(2), 1.1641533e-9);
        assert_eq!(i32_to_f32(1), 6.9849204e-10);
        assert_eq!(i32_to_f32(0), 2.3283067e-10);
        assert_eq!(i32_to_f32(-1), -2.3283067e-10);
        assert_eq!(i32_to_f32(-2), -6.9849204e-10);
        assert_eq!(i32_to_f32(-3), -1.1641533e-9);
        assert_eq!(i32_to_f32(-4), -1.6298147e-9);
        assert_eq!(i32_to_f32(-5), -2.095476e-9);
        assert_eq!(i32_to_f32(-6), -2.5611373e-9);
        assert_eq!(i32_to_f32(-7), -3.0267988e-9);
        assert_eq!(i32_to_f32(-8), -3.49246e-9);
        assert_eq!(i32_to_f32(i32::MIN / 512), -0.001953125);
        assert_eq!(i32_to_f32(i32::MIN / 256), -0.00390625);
        assert_eq!(i32_to_f32(i32::MIN / 128), -0.0078125);
        assert_eq!(i32_to_f32(i32::MIN / 64), -0.015625);
        assert_eq!(i32_to_f32(i32::MIN / 32), -0.03125);
        assert_eq!(i32_to_f32(i32::MIN / 16), -0.0625);
        assert_eq!(i32_to_f32(i32::MIN / 8), -0.125);
        assert_eq!(i32_to_f32(i32::MIN / 4), -0.25);
        assert_eq!(i32_to_f32(i32::MIN / 2), -0.5);
        assert_eq!(i32_to_f32(i32::MIN), -1.0);
    }

    #[test]
    fn float_to_unsigned() {
        for i in 0..10 {
            assert_eq!(
                f32_to_u32(0.5f32.powi(i.try_into().unwrap())),
                u32::MAX / 2u32.pow(i),
            );
        }

        assert_eq!(f32_to_u32(0.0), 0);
        assert_eq!(f32_to_u32(2.3283067e-10), 1);
        assert_eq!(f32_to_u32(4.6566134e-10), 2);
        assert_eq!(f32_to_u32(6.9849204e-10), 3);
        assert_eq!(f32_to_u32(9.313227e-10), 4);
        assert_eq!(f32_to_u32(1.1641533e-9), 5);
        assert_eq!(f32_to_u32(1.3969841e-9), 6);
        assert_eq!(f32_to_u32(1.6298147e-9), 7);
        assert_eq!(f32_to_u32(1.8626454e-9), 8);
        assert_eq!(f32_to_u32(2.095476e-9), 9);
    }

    #[test]
    fn float_to_signed() {
        assert_eq!(f32_to_i32(0.0), 0);
        assert_eq!(f32_to_i32(1.0), i32::MAX);
        assert_eq!(f32_to_i32(0.5), i32::MAX / 2);
        assert_eq!(f32_to_i32(0.25), i32::MAX / 4);
        assert_eq!(f32_to_i32(0.125), i32::MAX / 8);
        assert_eq!(f32_to_i32(0.0625), i32::MAX / 16);
        assert_eq!(f32_to_i32(0.03125), i32::MAX / 32);
        assert_eq!(f32_to_i32(0.015625), i32::MAX / 64);
        assert_eq!(f32_to_i32(0.0078125), i32::MAX / 128);
        assert_eq!(f32_to_i32(0.00390625), i32::MAX / 256);
        assert_eq!(f32_to_i32(0.001953125), i32::MAX / 512);
        assert_eq!(f32_to_i32(3.49246e-9), 7);
        assert_eq!(f32_to_i32(3.0267988e-9), 6);
        assert_eq!(f32_to_i32(2.5611373e-9), 5);
        assert_eq!(f32_to_i32(2.095476e-9), 4);
        assert_eq!(f32_to_i32(1.6298147e-9), 3);
        assert_eq!(f32_to_i32(1.1641533e-9), 2);
        assert_eq!(f32_to_i32(6.9849204e-10), 1);
        assert_eq!(f32_to_i32(2.3283067e-10), 0);
        assert_eq!(f32_to_i32(-2.3283067e-10), -1);
        assert_eq!(f32_to_i32(-6.9849204e-10), -2);
        assert_eq!(f32_to_i32(-1.1641533e-9), -3);
        assert_eq!(f32_to_i32(-1.6298147e-9), -4);
        assert_eq!(f32_to_i32(-2.095476e-9), -5);
        assert_eq!(f32_to_i32(-2.5611373e-9), -6);
        assert_eq!(f32_to_i32(-3.0267988e-9), -7);
        assert_eq!(f32_to_i32(-3.49246e-9), -8);
        assert_eq!(f32_to_i32(-0.001953125), i32::MIN / 512);
        assert_eq!(f32_to_i32(-0.00390625), i32::MIN / 256);
        assert_eq!(f32_to_i32(-0.0078125), i32::MIN / 128);
        assert_eq!(f32_to_i32(-0.015625), i32::MIN / 64);
        assert_eq!(f32_to_i32(-0.03125), i32::MIN / 32);
        assert_eq!(f32_to_i32(-0.0625), i32::MIN / 16);
        assert_eq!(f32_to_i32(-0.125), i32::MIN / 8);
        assert_eq!(f32_to_i32(-0.25), i32::MIN / 4);
        assert_eq!(f32_to_i32(-0.5), i32::MIN / 2);
        assert_eq!(f32_to_i32(-1.0), i32::MIN);
    }
}
