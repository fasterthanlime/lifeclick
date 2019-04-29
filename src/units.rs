use num_format::{Locale, ToFormattedString};
use std::fmt;
use std::ops;

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Souls(pub i64);

impl Souls {
    pub const K: Self = Self(1_000);
    pub const M: Self = Self(1_000_000);
    pub const B: Self = Self(1_000_000_000);
    pub const T: Self = Self(1_000_000_000_000);

    pub fn float(self) -> f64 {
        let Souls(f) = self;
        f as f64
    }

    pub fn times(self, x: i64) -> Self {
        Self(self.0 * x)
    }
}

impl From<i64> for Souls {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl Into<i64> for Souls {
    fn into(self) -> i64 {
        self.0
    }
}

impl ops::Mul for Souls {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl ops::Add for Souls {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Souls {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul<i64> for Souls {
    type Output = Self;

    fn mul(self, rhs: i64) -> Souls {
        Souls(self.0 * rhs)
    }
}

impl ops::Div<i64> for Souls {
    type Output = Self;

    fn div(self, rhs: i64) -> Souls {
        Souls(self.0 / rhs)
    }
}

impl ops::Mul<Souls> for i64 {
    type Output = Souls;

    fn mul(self, rhs: Souls) -> Souls {
        Souls(self * rhs.0)
    }
}

impl ops::AddAssign for Souls {
    fn add_assign(&mut self, rhs: Self) {
        (*self).0 += rhs.0
    }
}

impl ops::SubAssign for Souls {
    fn sub_assign(&mut self, rhs: Self) {
        (*self).0 -= rhs.0
    }
}

impl fmt::Display for Souls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = *self;
        if n >= Self::T {
            let q = self.float() / Self::T.float();
            write!(f, "{:.2} T", q)
        } else if n >= Self::B {
            let q = self.float() / Self::B.float();
            write!(f, "{:.2} B", q)
        } else if n >= Self::M {
            let q = self.float() / Self::M.float();
            write!(f, "{:.2} M", q)
        } else {
            write!(f, "{}", self.0.to_formatted_string(&Locale::en))
        }
    }
}
