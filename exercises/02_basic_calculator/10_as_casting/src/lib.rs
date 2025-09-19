// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        // When casting from a smaller integer type to a larger one
        // (e.g., u16 to u32), it's called a "widening" or "upcasting" conversion.
        // This is a safe operation because the larger type can represent every
        // possible value of the smaller type. The numeric value is preserved.
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8_transmute() {
        // This is an example of a "transmuting" conversion, where the raw bits
        // of a value are reinterpreted as a different type.

        // The `u8` value `255` is `11111111` in binary.

        // When we cast it to an `i8`, the bits don't change. Rust simply
        // reinterprets `11111111` as a signed 8-bit integer.
        // Using the "two's complement" system, this bit pattern represents `-1`.

        // The compiler would normally error on `255 as i8` because 255 doesn't
        // fit in the i8 range (-128 to 127). We allow it here for demonstration.
        let casted_value = {
            #[allow(overflowing_literals)]
            let value = 255 as i8;
            value
        };
        assert_eq!(casted_value, -1);
    }

    #[test]
    fn bool_to_u8() {
        // Booleans can be cast to integer types.
        // `true` becomes `1` and `false` becomes `0`.
        assert_eq!(true as u8, 1u8);
    }
}
