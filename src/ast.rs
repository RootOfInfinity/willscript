use crate::lexer::Operator;

#[derive(Clone, Debug)]
pub enum ExprAST {
    Variable(String),
    Number(i32),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
}

#[derive(Clone, Debug)]
pub struct FunctionAST {
    proto: PrototypeAST,
    body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct PrototypeAST {
    name: String,
    args: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(Assignment),
    If(IfBlock),
    Call(ExprAST),
    Built(BuiltIn),
}

#[derive(Clone, Debug)]
pub struct Assignment {
    is_declaration: bool,
    variable: ExprAST,
    right_hand: ExprAST,
}

#[derive(Clone, Debug)]
pub struct IfBlock {
    conditional: ExprAST,
    body: Vec<Assignment>,
}

#[derive(Clone, Debug)]
pub enum BuiltIn {
    Print(ExprAST),
    // these two are only with variables
    Input(ExprAST),
    Drop(ExprAST),
}
