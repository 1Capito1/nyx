use derive_more::{Add, AddAssign, Display, From, Into, Mul, Sub};

use super::{CheckedSub, Offset};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Add, Mul, Display, From, Into, Sub)]
pub(crate) struct Rank(pub u8);

impl CheckedSub for Rank {
    type Output = Self;
    fn checked_sub(&self, rhs: Self) -> Option<Self::Output> {
        self.0.checked_sub(rhs.0).map(Rank)
    }
}

impl Offset for Rank {
    type Output = Rank;
    fn offset(&self, delta: i8) -> Option<Self::Output> {
        let base = self.0 as i8 + delta;
        if (0..=7).contains(&base) {
            Some(Rank(base as u8))
        } else {
            None
        }
    }
}

impl Rank {
    pub(crate) fn diff(&self, rhs: Rank) -> Rank {
        Rank(self.0.abs_diff(rhs.0))
    }
}
