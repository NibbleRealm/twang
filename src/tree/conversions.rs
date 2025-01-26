/// Convert [`u32`] fraction to [`f32`] (ranged 0 to 1).
pub(crate) fn u32_to_f32(fraction: u32) -> f32 {
    // Check if fraction is 0
    let nonzero = u32::from(fraction != 0);
    // Calculate leading zeros
    let leading_zeros = nonzero * fraction.leading_zeros();
    // Remove leading zeros to subtract from exponent
    let fraction = fraction << leading_zeros;
    // Truncate to 24-bit fraction
    let fraction = fraction >> 8;
    // Clear 24th (inferred 1) bit
    let fraction = fraction & !(1 << 23);
    // Calculate -127 bias exponent (minus inferred 1)
    let exponent = (nonzero * 126 - leading_zeros) << 23;

    // Scale up (u32 max is 2³² - 1, and we want 2³²)
    f32::from_bits(exponent | fraction)
        * f32::from_bits(0b111111100000000000000000000001)
}

/// Convert signed 32-bit integer counter to floating point (-1 to 1)
pub(crate) fn int_to_float(int: i32) -> f32 {
    // Split sign and magnitude from signed integer
    let (uint, sign_bit) = if int < 0 {
        (reinterpret_unsigned(-1 - int), 1 << 31)
    } else {
        (reinterpret_unsigned(int), 0)
    };
    // Remove sign bit, so that it's not counted in leading zeros, add 1 for
    // rounding accuracy
    let fraction = (uint << 1) + 1;
    // Calculate leading zeros including inferred 1.
    let leading_zeros = fraction.leading_zeros();
    // Remove leading zeros to subtract from exponent
    let fraction = if leading_zeros >= 31 {
        0
    } else {
        // Remove inferred one in addition to leading zeros
        fraction << (leading_zeros + 1)
    };
    // Convert to 24-bit fraction
    let fraction = fraction >> 8;
    // Round up from extra half
    let fraction = fraction + (fraction & 1);
    // Remove zeroed-out bit, bringing fraction to 23/24 bits
    let fraction = fraction >> 1;
    // Clear 24th bit
    let fraction = fraction & !(1 << 23);
    // Calculate -127 bias exponent
    let exponent = (127 - leading_zeros) << 23;

    f32::from_bits(sign_bit | exponent | fraction)
}

/// Convert floating point (-1 to 1) to signed 32-bit integer
pub(crate) fn float_to_int(float: f32) -> i32 {
    // Infinity, 0, subnormal and NaN are all treated as 0
    let float = if !float.is_normal() {
        0u32
    } else {
        float.clamp(-1.0, 1.0).to_bits()
    };
    // Check if float is negative
    let is_negative = (float & (1 << 31)) != 0;
    // Convert to 23-bit fixed point fraction
    let int = (float << 9) >> 9;
    // Add floating point inferred digit 1.x, bringing to 24 bits
    let int = int | (1 << 23);
    // Extract 8-bit exponent
    let exponent = (float << 1) >> 24;
    // Calculate positive and negative shift
    let shift = 120u32.saturating_sub(exponent);
    let shift_back = exponent.saturating_sub(119);
    // Apply calculated exponential shift
    let int = (int << shift_back).saturating_sub(1);
    let int = if shift >= 32 { 0 } else { int >> shift };
    let int = reinterpret_signed(int);

    if is_negative {
        -1 - int
    } else {
        int
    }
}

#[inline(always)]
fn reinterpret_signed(int: u32) -> i32 {
    i32::from_ne_bytes(int.to_ne_bytes())
}

#[inline(always)]
fn reinterpret_unsigned(int: i32) -> u32 {
    u32::from_ne_bytes(int.to_ne_bytes())
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

        //
        assert_eq!(u32_to_f32(0), 0.0);
        assert_eq!(u32_to_f32(1), 2.3283067e-10);
        assert_eq!(u32_to_f32(2), 4.6566134e-10);
        assert_eq!(u32_to_f32(3), 6.9849204e-10);
        assert_eq!(u32_to_f32(4), 9.313227e-10);
        assert_eq!(u32_to_f32(5), 1.1641533e-9);
        assert_eq!(u32_to_f32(6), 1.3969841e-9);
        assert_eq!(u32_to_f32(7), 1.6298147e-9);
        assert_eq!(u32_to_f32(8), 1.8626454e-9);
    }

    #[test]
    fn conversions() {
        // Since there are more negative than positive integers, zeros are not
        // an exact match between integer and floating point
        assert_eq!(int_to_float(i32::MAX), 1.0);
        assert_eq!(int_to_float(i32::MAX / 2), 0.5);
        assert_eq!(int_to_float(i32::MAX / 4), 0.25);
        assert_eq!(int_to_float(i32::MAX / 8), 0.125);
        assert_eq!(int_to_float(i32::MAX / 16), 0.0625);
        assert_eq!(int_to_float(i32::MAX / 32), 0.03125);
        assert_eq!(int_to_float(i32::MAX / 64), 0.015625);
        assert_eq!(int_to_float(i32::MAX / 128), 0.0078125);
        assert_eq!(int_to_float(i32::MAX / 256), 0.0078124995); // 0.00390625);
        assert_eq!(int_to_float(i32::MAX / 512), 0.0039062495); // 0.001953125);
        assert_eq!(int_to_float(7), 6.9849193e-9);
        assert_eq!(int_to_float(6), 6.0535967e-9);
        assert_eq!(int_to_float(5), 5.122274e-09);
        assert_eq!(int_to_float(4), 4.1909516e-9);
        assert_eq!(int_to_float(3), 3.259629e-9);
        assert_eq!(int_to_float(2), 2.3283064e-9);
        assert_eq!(int_to_float(1), 1.3969839e-9);
        assert_eq!(int_to_float(0), 2.0f32.powf(-31.0));
        assert_eq!(int_to_float(-1), -2.0f32.powf(-31.0));
        assert_eq!(int_to_float(-2), -1.3969839e-9);
        assert_eq!(int_to_float(-3), -2.3283064e-9);
        assert_eq!(int_to_float(-4), -3.259629e-9);
        assert_eq!(int_to_float(-5), -4.1909516e-9);
        assert_eq!(int_to_float(-6), -5.122274e-09);
        assert_eq!(int_to_float(-7), -6.0535967e-9);
        assert_eq!(int_to_float(-8), -6.9849193e-9);
        assert_eq!(int_to_float(i32::MIN / 512), -0.0039062495); // -0.001953125);
        assert_eq!(int_to_float(i32::MIN / 256), -0.0078124995); // -0.00390625);
        assert_eq!(int_to_float(i32::MIN / 128), -0.0078125);
        assert_eq!(int_to_float(i32::MIN / 64), -0.015625);
        assert_eq!(int_to_float(i32::MIN / 32), -0.03125);
        assert_eq!(int_to_float(i32::MIN / 16), -0.0625);
        assert_eq!(int_to_float(i32::MIN / 8), -0.125);
        assert_eq!(int_to_float(i32::MIN / 4), -0.25);
        assert_eq!(int_to_float(i32::MIN / 2), -0.5);
        assert_eq!(int_to_float(i32::MIN), -1.0);

        assert_eq!(float_to_int(0.0), 0);
        assert_eq!(float_to_int(1.0), i32::MAX);
        assert_eq!(float_to_int(0.5), i32::MAX / 2);
        assert_eq!(float_to_int(0.25), i32::MAX / 4);
        assert_eq!(float_to_int(0.125), i32::MAX / 8);
        assert_eq!(float_to_int(0.0625), i32::MAX / 16);
        assert_eq!(float_to_int(0.03125), i32::MAX / 32);
        assert_eq!(float_to_int(0.015625), i32::MAX / 64);
        assert_eq!(float_to_int(0.0078125), i32::MAX / 128);
        assert_eq!(float_to_int(0.00390625), i32::MAX / 512); // / 256);
        assert_eq!(float_to_int(0.001953125), i32::MAX / 1024); // / 512);
        assert_eq!(float_to_int(6.9849193e-9), 7);
        assert_eq!(float_to_int(6.0535967e-9), 6);
        assert_eq!(float_to_int(5.122274e-09), 5);
        assert_eq!(float_to_int(4.1909516e-9), 4);
        assert_eq!(float_to_int(3.259629e-9), 3);
        assert_eq!(float_to_int(2.3283064e-9), 2);
        assert_eq!(float_to_int(1.3969839e-9), 1);
        assert_eq!(float_to_int(2.0f32.powf(-31.0)), 0);
        assert_eq!(float_to_int(-2.0f32.powf(-31.0)), -1);
        assert_eq!(float_to_int(-1.3969839e-9), -2);
        assert_eq!(float_to_int(-2.3283064e-9), -3);
        assert_eq!(float_to_int(-3.259629e-9), -4);
        assert_eq!(float_to_int(-4.1909516e-9), -5);
        assert_eq!(float_to_int(-5.122274e-09), -6);
        assert_eq!(float_to_int(-6.0535967e-9), -7);
        assert_eq!(float_to_int(-6.9849193e-9), -8);
        assert_eq!(float_to_int(-0.001953125), i32::MIN / 1024); // / 512);
        assert_eq!(float_to_int(-0.00390625), i32::MIN / 512); // / 256);
        assert_eq!(float_to_int(-0.0078125), i32::MIN / 128);
        assert_eq!(float_to_int(-0.015625), i32::MIN / 64);
        assert_eq!(float_to_int(-0.03125), i32::MIN / 32);
        assert_eq!(float_to_int(-0.0625), i32::MIN / 16);
        assert_eq!(float_to_int(-0.125), i32::MIN / 8);
        assert_eq!(float_to_int(-0.25), i32::MIN / 4);
        assert_eq!(float_to_int(-0.5), i32::MIN / 2);
        assert_eq!(float_to_int(-1.0), i32::MIN);
    }
}
