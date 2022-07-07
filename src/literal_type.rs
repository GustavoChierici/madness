use std::ops::{Add, Sub, Mul, Div};

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

// impl PartialEq<LiteralType> for bool {
//     fn eq(&self, other: &LiteralType) -> bool {
//         match other {
//             LiteralType::Bool(b) => b == self,
//             LiteralType::Int(i) => (*i != 0) == *self
//         }
//     }

//     fn ne(&self, other: &LiteralType) -> bool {
//         match other {
//             LiteralType::Bool(b) => b != self,
//             LiteralType::Int(i) => (*i != 0) != *self
//         }
//     }
// }

// impl PartialEq<LiteralType> for i64 {
//     fn eq(&self, other: &LiteralType) -> bool {
//         match other {
//             LiteralType::Bool(b) => *b == (*self != 0),
//             LiteralType::Int(i) => i == self
//         }
//     }

//     fn ne(&self, other: &LiteralType) -> bool {
//         match other {
//             LiteralType::Bool(b) => *b != (*self != 0),
//             LiteralType::Int(i) => i != self
//         }
//     }
// }

// impl PartialEq<i64> for LiteralType {
//     fn eq(&self, other: &i64) -> bool {
//         match self {
//             LiteralType::Bool(b) => *b == (*other != 0),
//             LiteralType::Int(i) => (*i != 0) == (*other != 0)
//         }
//     }

//     fn ne(&self, other: &i64) -> bool {
//         match self {
//             LiteralType::Bool(b) => *b != (*other != 0),
//             LiteralType::Int(i) => (*i != 0) != (*other != 0)
//         }
//     }
// }

// impl PartialEq<bool> for LiteralType {
//     fn eq(&self, other: &bool) -> bool {
//         match self {
//             LiteralType::Bool(b) => b == other,
//             LiteralType::Int(i) => (*i != 0) == *other 
//         }
//     }

//     fn ne(&self, other: &bool) -> bool {
//         match self {
//             LiteralType::Bool(b) => b != other,
//             LiteralType::Int(i) => (*i != 0) != *other 
//         }
//     }
// }

// impl PartialEq for LiteralType {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             Self::Bool(b) => b == other,
//             Self::Int(i) => i == other
//         }
//     }

//     fn ne(&self, other: &Self) -> bool {
//         match self {
//             Self::Bool(b) => b != other,
//             Self::Int(i) => i != other
//         }
//     }
// }
