use std::{cell::RefCell, rc::Rc};

pub mod literal_type;

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

fn main() {
    let expr1 = Rc::new(RefCell::new(
        Expr::IfElseExpr(
            Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::Bool(true))))),
            Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::Bool(true))))),
            Some(Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::Bool(false))))))
        )
    ));

    let expr2 = Rc::new(RefCell::new(
        Expr::IfElseExpr(
            expr1.clone(),
            Rc::new(RefCell::new(Expr::ArithmeticExpr(
                Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::String("Hello".to_string()))))),
                std::ops::Add::add,
                Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::String("World".to_string())))))
            ))),
            Some(Rc::new(RefCell::new(Expr::Literal(Some(literal_type::LiteralType::Int(2)))))),
        )
    ));

    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);
    expr2.borrow_mut().evaluate();
    println!("1: {:?}", expr1);
    println!("2: {:?}", expr2);
}