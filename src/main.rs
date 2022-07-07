use std::{cell::RefCell, rc::Rc};

pub mod literal_type;

#[derive(Debug)]
pub enum Expr {
    ArithmeticExpr(Rc<RefCell<Expr>>, String, Rc<RefCell<Expr>>),
    Literal(literal_type::LiteralType),
    IfElseExpr(Rc<RefCell<Expr>>, Rc<RefCell<Expr>>, Option<Rc<RefCell<Expr>>>),
} 

impl Expr {
    pub fn evaluate(&mut self) -> literal_type::LiteralType {
        let value = match self {
            Self::ArithmeticExpr(lhs, op, rhs) => {
                match op.as_str() {
                    "+" => lhs.borrow_mut().evaluate() + rhs.borrow_mut().evaluate(),
                    "-" => lhs.borrow_mut().evaluate() - rhs.borrow_mut().evaluate(),
                    "*" => lhs.borrow_mut().evaluate() * rhs.borrow_mut().evaluate(),
                    "/" => lhs.borrow_mut().evaluate() / rhs.borrow_mut().evaluate(),
                    _ => todo!("Invalid arithmetic expression")
                }
            },
            Self::Literal(value) => value.clone(),
            Self::IfElseExpr(cond, expr_if_true, expr_if_false) => {
                if cond.borrow_mut().evaluate().into() {
                    expr_if_true.borrow_mut().evaluate()
                } else {
                    if expr_if_false.is_some() {
                        expr_if_false.as_ref().unwrap().borrow_mut().evaluate()
                    } else {
                        literal_type::LiteralType::Int(0)
                    }
                }
            }
        };

        *self = Expr::Literal(value.clone());
        value
    }
}

fn main() {
    let expr1 = Rc::new(RefCell::new(
        Expr::IfElseExpr(
            Rc::new(RefCell::new(Expr::Literal(literal_type::LiteralType::Bool(true)))),
            Rc::new(RefCell::new(Expr::Literal(literal_type::LiteralType::Bool(true)))),
            Some(Rc::new(RefCell::new(Expr::Literal(literal_type::LiteralType::Bool(false)))))
        )
    ));

    let expr2 = Rc::new(RefCell::new(
        Expr::IfElseExpr(
            expr1.clone(),
            Rc::new(RefCell::new(Expr::Literal(literal_type::LiteralType::Int(1)))),
            Some(Rc::new(RefCell::new(Expr::Literal(literal_type::LiteralType::Int(2))))),
        )
    ));

    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);
    expr2.borrow_mut().evaluate();
    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);
}