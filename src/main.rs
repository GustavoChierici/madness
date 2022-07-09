use std::{cell::RefCell, rc::Rc, ops::Add};

pub mod literal_type;
use crate::literal_type::ToLiteral;

#[derive(Debug)]
pub enum Expr {
    ArithmeticExpr(Rc<RefCell<Expr>>, fn(literal_type::LiteralType, literal_type::LiteralType) -> literal_type::LiteralType, Rc<RefCell<Expr>>),
    Literal(Option<literal_type::LiteralType>),
    IfElseExpr(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Option<Rc<RefCell<Expr>>>),
} 

impl Expr {
    pub fn evaluate(&mut self) -> Option<literal_type::LiteralType> {
        let value = match self {
            Self::ArithmeticExpr(lhs, op, rhs) => {
                Some(op(lhs.borrow_mut().evaluate().expect("Cannot operate on None"), rhs.borrow_mut().evaluate().expect("Cannot operate on None")))
            },
            Self::Literal(value) => value.clone(),
            Self::IfElseExpr(cond, expr_if_true, expr_if_false) => {
                if cond.borrow_mut().evaluate().unwrap_or(literal_type::LiteralType::Bool(false)).into() {
                    expr_if_true.borrow_mut().evaluate()
                } else {
                    if expr_if_false.is_some() {
                        expr_if_false.as_ref().unwrap().borrow_mut().evaluate()
                    } else {
                        None
                    }
                }
            }
        };

        *self = Expr::Literal(value.clone());
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

macro_rules! expr {
    ($(None)?) => { literal!(None) };
    ($lit:expr) => { literal!($lit) };
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
    }
}

fn main() {
    let expr1 = expr!{
        If true => true;
        Else => false
    };

    let expr2 = expr!{
        If expr1.clone() => expr!("Hello", Add::add, " World!");
        Else => 2
    };

    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);
    expr2.borrow_mut().evaluate();
    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);

    println!("{:?}", expr!{
        If true => expr!(5, Add::add, 7);
        Else => 12
    }.borrow_mut().evaluate());
}