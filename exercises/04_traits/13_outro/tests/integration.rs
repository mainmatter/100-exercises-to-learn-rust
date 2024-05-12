use outro_03::SaturatingU16;

#[test]
fn test_saturating_u16() {
    let a: SaturatingU16 = (&10u8).into();
    let b: SaturatingU16 = 5u8.into();
    let c: SaturatingU16 = u16::MAX.into();
    let d: SaturatingU16 = (&1u16).into();

    assert_eq!(a + b, SaturatingU16::from(15u16));
    assert_eq!(a + c, SaturatingU16::from(u16::MAX));
    assert_eq!(a + d, SaturatingU16::from(11u16));
    assert_eq!(a + a, 20u16);
    assert_eq!(a + 5u16, 15u16);
    assert_eq!(a + &u16::MAX, SaturatingU16::from(u16::MAX));
}
