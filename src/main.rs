// 1- Análise Léxica (lexer)
// 2- Análise Sintática (parser) - AST
// Compilador: gerar código de baixo nível (assembly)
// Interpretador: executar instruções com base na AST
// Converter a AST em uma representação intermediária (LLVM/bytecode (JVM))
// Máquina virtual que roda a sua linguagem

#[derive(Debug)]
struct Expr {
    lhs: Option<i64>,
    op: Option<String>,
    rhs: Option<i64> 
} 

impl Expr {
    pub fn evaluate(&mut self) -> i64 {
        if self.op.is_none() { 
            return self.lhs.unwrap();
        }

        let value = match self.op.as_ref().unwrap().as_str() {
            "+" => self.lhs.unwrap() + self.rhs.unwrap(),
            "-" => self.lhs.unwrap() - self.rhs.unwrap(),
            "*" => self.lhs.unwrap() * self.rhs.unwrap(),
            "/" => self.lhs.unwrap() / self.rhs.unwrap(),
            _ => todo!("a")
        };

        self.lhs = Some(value);
        self.rhs = None;
        self.op = None;

        return value;
    }
}

fn main() {
    let mut expr = Expr {
        lhs: Some(5),
        op: Some("-".to_string()),
        rhs: Some(3)
    };
    println!("{:?}", expr);
    expr.evaluate();
    println!("{:?}", expr);
    expr.evaluate();
    println!("{:?}", expr);
}

