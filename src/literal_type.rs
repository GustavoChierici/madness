use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Int(i64),
    Bool(bool)
}

impl Add<LiteralType> for i64 {
    type Output = LiteralType;

    fn add(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self + a),
            LiteralType::Bool(b) => LiteralType::Int(self + b as i64)
        }
    }
}

impl Add<LiteralType> for bool {
    type Output = LiteralType;

    fn add(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self as i64 + a),
            LiteralType::Bool(b) => LiteralType::Int(self as i64 + b as i64)
        }
    }
}

impl Add for LiteralType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i + other,
            Self::Bool(b) => b + other,
        }
    }
}

impl Sub<LiteralType> for i64 {
    type Output = LiteralType;

    fn sub(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self - a),
            LiteralType::Bool(b) => LiteralType::Int(self - b as i64)
        }
    }
}

impl Sub<LiteralType> for bool {
    type Output = LiteralType;

    fn sub(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self as i64 - a),
            LiteralType::Bool(b) => LiteralType::Int(self as i64 - b as i64)
        }
    }
}

impl Sub for LiteralType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i - other,
            Self::Bool(b) => b - other,
        }
    }
}

impl Mul<LiteralType> for i64 {
    type Output = LiteralType;

    fn mul(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self * a),
            LiteralType::Bool(_) => panic!("Can't multiply a bool")
        }
    }
}

impl Mul for LiteralType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i * other,
            Self::Bool(_) => panic!("Can't multiply a bool"),
        }
    }
}

impl Div<LiteralType> for i64 {
    type Output = LiteralType;

    fn div(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self / a),
            LiteralType::Bool(_) => panic!("Can't divide a bool")
        }
    }
}

impl Div for LiteralType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i / other,
            Self::Bool(_) => panic!("Can't divide a bool"),
        }
    }
}

impl From<LiteralType> for bool {
    fn from(lit: LiteralType) -> Self {
        match lit {
            LiteralType::Bool(b) => b,
            LiteralType::Int(i) => i != 0
        }
    }
}

impl From<LiteralType> for i64 {
    fn from(lit: LiteralType) -> Self {
        match lit {
            LiteralType::Bool(b) => b.into(),
            LiteralType::Int(i) => i
        }
    }
}

impl Neg for LiteralType {
    type Output = LiteralType;

    fn neg(self) -> Self::Output {
        match self {
            LiteralType::Bool(b) => LiteralType::Int(-(b as i64)),
            LiteralType::Int(i) => LiteralType::Int(-i)
        }
    }
}