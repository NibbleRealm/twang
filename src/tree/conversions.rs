/// Convert unsigned 32-bit integer counter to floating point (-1 to 1)
pub(crate) fn int_to_float(int: i32) -> f32 {
    // Split sign and magnitude from signed integer
    let (uint, sign_bit) = if int < 0 {
        ((-1 - int) as u32, 1 << 31)
    } else {
        (int as u32, 0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        // Since there are more negative than positive integers, zeros are not
        // an exact match between integer and floating point
        assert_eq!(int_to_float(i32::MAX), 1.0);
        assert_eq!(int_to_float(i32::MAX / 2), 0.5);
        assert_eq!(int_to_float(i32::MAX / 4), 0.25);
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
        assert_eq!(int_to_float(i32::MIN / 4), -0.25);
        assert_eq!(int_to_float(i32::MIN / 2), -0.5);
        assert_eq!(int_to_float(i32::MIN), -1.0);
    }
}
