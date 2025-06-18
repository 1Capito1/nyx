use derive_more::{Add, AddAssign, Display, From, Into, Mul, Sub};
use super::CheckedSub;
use super::Offset;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Add, Mul, Display, From, Into, Sub, AddAssign)]
pub(crate) struct File(pub u8);


impl Offset for File {
    type Output = File;
    fn offset(&self, delta: i8) -> Option<Self::Output> {
        let base = self.0 as i8 + delta;
        if (0..=7).contains(&base) {
            Some(File(base as u8))
        } else {
            None
        }
    }
}

impl CheckedSub for File {
    type Output = Self;
    fn checked_sub(&self, rhs: Self) -> Option<Self::Output> {
        self.0.checked_sub(rhs.0).map(File)
    }
}

