use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Int(i64),
    Bool(bool),
    String(String)
}

impl Add<LiteralType> for i64 {
    type Output = LiteralType;

    fn add(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self + a),
            LiteralType::Bool(b) => LiteralType::Int(self + b as i64),
            LiteralType::String(s) => LiteralType::String(self.to_string() + s.as_str())
        }
    }
}

impl Add<LiteralType> for bool {
    type Output = LiteralType;

    fn add(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self as i64 + a),
            LiteralType::Bool(b) => LiteralType::Int(self as i64 + b as i64),
            LiteralType::String(s) => LiteralType::String(self.to_string() + s.as_str())
        }
    }
}

impl Add<LiteralType> for String {
    type Output = LiteralType;

    fn add(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::String(self + a.to_string().as_str()),
            LiteralType::Bool(b) => LiteralType::String(self + b.to_string().as_str()),
            LiteralType::String(s) => LiteralType::String(self + s.as_str())
        }
    }
}

impl Add for LiteralType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i + other,
            Self::Bool(b) => b + other,
            Self::String(s) => s + other,
        }
    }
}

impl Sub<LiteralType> for i64 {
    type Output = LiteralType;

    fn sub(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self - a),
            LiteralType::Bool(b) => LiteralType::Int(self - b as i64),
            LiteralType::String(_) => panic!("String cannot be subtracted")
        }
    }
}

impl Sub<LiteralType> for bool {
    type Output = LiteralType;

    fn sub(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self as i64 - a),
            LiteralType::Bool(b) => LiteralType::Int(self as i64 - b as i64),
            LiteralType::String(_) => panic!("Can't subract a string")
        }
    }
}

impl Sub for LiteralType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i - other,
            Self::Bool(b) => b - other,
            Self::String(_) => panic!("Can't subtract a string")
        }
    }
}

impl Mul<LiteralType> for i64 {
    type Output = LiteralType;

    fn mul(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self * a),
            LiteralType::Bool(_) => panic!("Can't multiply a bool"),
            LiteralType::String(_) => panic!("Can't multiply a string")
        }
    }
}

impl Mul for LiteralType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i * other,
            Self::Bool(_) => panic!("Can't multiply a bool"),
            LiteralType::String(_) => panic!("Can't multiply string")
        }
    }
}

impl Div<LiteralType> for i64 {
    type Output = LiteralType;

    fn div(self, other: LiteralType) -> LiteralType {
        match other {
            LiteralType::Int(a) => LiteralType::Int(self / a),
            LiteralType::Bool(_) => panic!("Can't divide a bool"),
            LiteralType::String(_) => panic!("Can't divide a string")
        }
    }
}

impl Div for LiteralType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            Self::Int(i) => i / other,
            Self::Bool(_) => panic!("Can't divide a bool"),
            Self::String(_) => panic!("Can't divide a string")
        }
    }
}

impl From<LiteralType> for bool {
    fn from(lit: LiteralType) -> Self {
        match lit {
            LiteralType::Bool(b) => b,
            LiteralType::Int(i) => i != 0,
            LiteralType::String(s) => s != ""
        }
    }
}

impl From<LiteralType> for i64 {
    fn from(lit: LiteralType) -> Self {
        match lit {
            LiteralType::Bool(b) => b.into(),
            LiteralType::Int(i) => i,
            LiteralType::String(s) => s.parse().unwrap()
        }
    }
}

impl Neg for LiteralType {
    type Output = LiteralType;

    fn neg(self) -> Self::Output {
        match self {
            LiteralType::Bool(b) => LiteralType::Int(-(b as i64)),
            LiteralType::Int(i) => LiteralType::Int(-i),
            LiteralType::String(_) => panic!("Can't neg a string")
        }
    }
}