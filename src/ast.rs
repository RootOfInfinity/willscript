use crate::lexer::Operator;

#[derive(Clone, Debug)]
enum ExprAST {
    Variable(String),
    Number(i32),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
}

#[derive(Clone, Debug)]
struct FunctionAST {
    proto: PrototypeAST,
    body: Vec<Statement>,
}

#[derive(Clone, Debug)]
struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

#[derive(Clone, Debug)]
enum Statement {
    Assign(Assignment),
    If(IfBlock),
    Call(ExprAST),
}

#[derive(Clone, Debug)]
struct Assignment {
    is_declaration: bool,
    variable: ExprAST,
    right_hand: ExprAST,
}

#[derive(Clone, Debug)]
struct IfBlock {
    conditional: ExprAST,
    body: Vec<Assignment>,
}
