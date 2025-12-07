// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        SaturatingU16 { value: *value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        SaturatingU16 {
            value: value as u16,
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> SaturatingU16 {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add((*other).value),
        }
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other),
        }
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(*other),
        }
    }
}

impl std::cmp::PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value.eq(other)
    }
}
