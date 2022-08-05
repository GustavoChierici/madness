#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use std::{cell::RefCell, rc::Rc, ops::{Add, Mul, Deref}, fs};

pub mod literal_type;

use literal_type::LiteralType;

use crate::literal_type::ToLiteral;



#[derive(Debug, Clone)]
pub enum LiteralOrVar {
    Literal(Option<literal_type::LiteralType>),
    Var(String, Rc<RefCell<Expr>>),
}

impl LiteralOrVar {
    pub fn value(&self) -> Option<LiteralType> {
        match self {
            LiteralOrVar::Literal(lit) => lit.clone(),
            LiteralOrVar::Var(_, value) => value.borrow_mut().evaluate().value()
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Literal(Option<literal_type::LiteralType>),
    Var(String, Rc<RefCell<Expr>>),
    VarDecl(String, Rc<RefCell<Expr>>),
    ArithmeticExpr(Rc<RefCell<Expr>>, fn(literal_type::LiteralType, literal_type::LiteralType) -> literal_type::LiteralType, Rc<RefCell<Expr>>),
    IfElseExpr(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Option<Rc<RefCell<Expr>>>),
    FunctionCall(fn(Vec<Option<literal_type::LiteralType>>) -> Rc<RefCell<Expr>>, Vec<Rc<RefCell<Expr>>>),
    BooleanExpr(Rc<RefCell<Expr>>, fn(literal_type::LiteralType, literal_type::LiteralType) -> literal_type::LiteralType, Rc<RefCell<Expr>>),
} 

static mut VARS: Vec<LiteralOrVar> = vec!();

impl Expr {
    pub fn evaluate(&mut self) -> LiteralOrVar {
        let value = match self {
            Self::ArithmeticExpr(lhs, op, rhs) => {
                LiteralOrVar::Literal(Some(op(lhs.borrow_mut().evaluate().value().expect("Cannot operate on none"), rhs.borrow_mut().evaluate().value().expect("Cannot operate on none"))))
            },
            Self::Literal(value) => LiteralOrVar::Literal(value.clone()),
            Self::IfElseExpr(cond, expr_if_true, expr_if_false) => {
                if cond.borrow_mut().evaluate().value().unwrap_or(literal_type::LiteralType::Bool(false)).into() {
                    expr_if_true.borrow_mut().evaluate()
                } else {
                    if expr_if_false.is_some() {
                        expr_if_false.as_ref().unwrap().borrow_mut().evaluate()
                    } else {
                        LiteralOrVar::Literal(None)
                    }
                }
            },
            Self::FunctionCall(function, args) => {
                function(args.iter().map(|x| x.borrow_mut().evaluate().value()).collect()).borrow_mut().evaluate()
            },
            Self::BooleanExpr(lhs, cmp, rhs) => {
                LiteralOrVar::Literal(Some(cmp(lhs.borrow_mut().evaluate().value().expect("Cannot operate on None"), rhs.borrow_mut().evaluate().value().expect("Cannot operate on None"))))
            },
            Expr::Var(_, value) => {
                value.borrow_mut().evaluate()
            },
            Expr::VarDecl(name, value) => {
                let var = LiteralOrVar::Var(name.to_string(), Rc::new(RefCell::new(Expr::Var(name.to_string(), value.clone()))));
                unsafe {
                    VARS.push(var.clone());
                }
                var
            }
        };

        match value.clone() {
            LiteralOrVar::Literal(lit) => {*self = Expr::Literal(lit);}
            _ => {}
        }
        value
    }
}

pub trait ToExpr {
    fn to_expr(&self) -> Rc<RefCell<Expr>>;
}

impl ToExpr for Rc<RefCell<Expr>> {
    fn to_expr(&self) -> Rc<RefCell<Expr>> {
        self.clone()
    }
}

impl<T> ToExpr for T where T: ToLiteral + Copy {
    fn to_expr(&self) -> Rc<RefCell<Expr>> {
        Rc::new(RefCell::new(Expr::Literal(Some(self.to_literal()))))
    }
}

impl ToExpr for Option<literal_type::LiteralType> {
    fn to_expr(&self) -> Rc<RefCell<Expr>> {
        Rc::new(RefCell::new(Expr::Literal(self.clone())))
    }
}

impl ToExpr for Option<&LiteralOrVar> {
    fn to_expr(&self) -> Rc<RefCell<Expr>> {
        match self.unwrap() {
            LiteralOrVar::Literal(lit) => lit.to_expr(),
            LiteralOrVar::Var(name, expr) => Rc::new(RefCell::new(Expr::Var(name.clone(), expr.clone())))
        }
    }
}

macro_rules! expr {
    ($(None)?) => { None.to_expr() };
    ($name:ident) => {
        unsafe {
            VARS.iter().find(|x| {
                match x {
                    LiteralOrVar::Var(name, _) => *name == stringify!($name).to_string(),
                    _ => false
                }
            })
        }
    };
    ($lit:expr) => { $lit.to_expr() };
    ($lhs:expr,$op:expr,$rhs:expr) => { 
        Rc::new(RefCell::new(Expr::ArithmeticExpr(
            $lhs.to_expr(), 
            $op,
            $rhs.to_expr()
        )))
    };
    (If $cond:expr => $if_true:expr; Else => $if_false:expr) => { 
        Rc::new(RefCell::new(Expr::IfElseExpr(
            $cond.to_expr(), 
            $if_true.to_expr(),
            Some($if_false.to_expr())
        )))
    };
    (If $cond:expr => $if_true:expr) => { 
        Rc::new(RefCell::new(Expr::IfElseExpr(
            $cond.to_expr(), 
            $if_true.to_expr(),
            None
        )))
    };
    (apply $fun:expr => $($args:expr),*) => {
        Rc::new(RefCell::new(Expr::FunctionCall(
            $fun,
            vec!($($args.to_expr()),*)
        )))
    };
    (Let $name:ident = $value:expr) => {
        Rc::new(RefCell::new(Expr::VarDecl(
            stringify!($name).to_string(),
            $value.to_expr()
        ))).borrow_mut().evaluate()
    }
}

#[derive(Debug)]
pub enum Token {
    Int(i64),
    PLUS,
}

fn main() {
    let mut file = fs::read_to_string("./test.mad").expect("File not found");

    let words: Vec<&str> = file.trim().split_whitespace().collect();

    let mut tokens: Vec<Token> = vec![];

    for word in words {
        let token = if word.parse::<i64>().is_ok() {
            Token::Int(word.parse::<i64>().unwrap())
        } else if word == "+" {
            Token::PLUS
        } else {
            panic!("Unkown word!")
        };

        tokens.push(token);
    }

    let mut ast = expr!(5);


    let mut it = tokens.iter().peekable();

    let mut token = it.next();
    while token.is_some() {
        ast = match token.unwrap() {
            Token::Int(i) => { 
                match it.peek().unwrap() {
                    Token::PLUS => {
                        it.next();
                        let rhs = it.next();

                        let i2 = match rhs.unwrap() {
                            Token::Int(i2) => i2,
                            _ => panic!("Unhandled")
                        };
                        expr!(i, Add::add, i2)
                    }
                    Token::Int(_) => panic!("Unhandled")
                }
            }
            Token::PLUS => {
                panic!("Unhandled")
            }
        };

        token = it.next();
    }

    print!("{:?}", ast.borrow_mut().evaluate());
}

// fn main() {
//     let a = expr!(Let a = 10);
//     let b = expr!(Let b = expr!(5, Mul::mul, 3));
//     let c = expr!(Let c = expr!(expr!(a), Add::add, expr!(b)));
//     let d = expr!(If expr!(c) => expr!(apply |x| {
//         let mut sum = 0.to_literal();
//         for e in x.iter() {
//             sum = sum + e.clone().unwrap();
//         }
//         Some(sum).to_expr()
//     } => expr!(a), expr!(b), expr!(c)); Else => "Else");

    
//     println!("a: {:?}\n", a);
//     println!("b: {:?}\n", b);
//     println!("c: {:?}\n", c);
//     println!("d: {:?}\n", d);
    
//     d.borrow_mut().evaluate();
    
//     println!("***AFTER EVALUATION***\n");
    
//     println!("a: {:?}\n", a);
//     println!("b: {:?}\n", b);
//     println!("c: {:?}\n", c);
//     println!("d: {:?}\n", d);
// }