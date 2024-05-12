// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        assert_eq!(47u16 as u32, todo!());
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_to_i8() {
        assert_eq!(255 as i8, todo!());
    }

    #[test]
    fn bool_to_u8() {
        assert_eq!(true as u8, todo!());
    }
}
