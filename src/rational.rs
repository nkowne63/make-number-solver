use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

// 有理数
#[derive(Debug, Clone, Copy, Eq)]
pub(crate) struct Rational {
    // 分子
    pub(crate) numerator: i32,
    // 分母
    pub(crate) denominator: i32,
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator != 1 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "{}", self.numerator)
        }
    }
}
impl Rational {
    pub(crate) fn new(numerator: i32, denominator: i32) -> Self {
        Rational {
            numerator,
            denominator,
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Rational {
        Rational {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Rational {
        Rational {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Rational {
        Rational {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Rational {
        Rational {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Rational {
                numerator: 1,
                denominator: 2,
            } + Rational {
                numerator: 1,
                denominator: 3,
            },
            Rational {
                numerator: 5,
                denominator: 6,
            }
        );
    }
    #[test]
    fn sub() {
        assert_eq!(
            Rational {
                numerator: 1,
                denominator: 2,
            } - Rational {
                numerator: 1,
                denominator: 3,
            },
            Rational {
                numerator: 1,
                denominator: 6,
            }
        );
    }
    #[test]
    fn mul() {
        assert_eq!(
            Rational {
                numerator: 1,
                denominator: 2,
            } * Rational {
                numerator: 1,
                denominator: 3,
            },
            Rational {
                numerator: 1,
                denominator: 6,
            }
        );
    }
    #[test]
    fn div() {
        assert_eq!(
            Rational {
                numerator: 1,
                denominator: 2,
            } / Rational {
                numerator: 1,
                denominator: 3,
            },
            Rational {
                numerator: 3,
                denominator: 2,
            }
        );
    }
}
