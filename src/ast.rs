use crate::lexer::Operator;

#[derive(Clone, Debug)]
pub enum ExprAST {
    Variable(String),
    Val(Value),
    BinOp(Operator, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Str(String),
    Int(i32),
}

#[derive(Clone, Debug)]
pub struct FunctionAST {
    pub proto: PrototypeAST,
    pub body: Vec<Statement>,
}
impl FunctionAST {
    pub fn new(proto: PrototypeAST, body: Vec<Statement>) -> Self {
        FunctionAST { proto, body }
    }
}

#[derive(Clone, Debug)]
pub struct PrototypeAST {
    pub name: String,
    pub args: Vec<String>,
}
impl PrototypeAST {
    pub fn new(name: String, args: Vec<String>) -> Self {
        PrototypeAST { name, args }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assign(Assignment),
    If(IfBlock),
    While(WhileBlock),
    Call(ExprAST),
    Built(BuiltIn),
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub is_declaration: bool,
    pub variable: ExprAST,
    pub right_hand: ExprAST,
}
impl Assignment {
    pub fn new(is_declaration: bool, variable: ExprAST, right_hand: ExprAST) -> Self {
        Assignment {
            is_declaration,
            variable,
            right_hand,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IfBlock {
    pub conditional: ExprAST,
    pub body: Vec<Statement>,
}
impl IfBlock {
    pub fn new(conditional: ExprAST, body: Vec<Statement>) -> Self {
        IfBlock { conditional, body }
    }
}

#[derive(Clone, Debug)]
pub struct WhileBlock {
    pub conditional: ExprAST,
    pub body: Vec<Statement>,
}
impl WhileBlock {
    pub fn new(conditional: ExprAST, body: Vec<Statement>) -> Self {
        WhileBlock { conditional, body }
    }
}

#[derive(Clone, Debug)]
pub enum BuiltIn {
    Print(ExprAST),
    Return(ExprAST),
    // these two are only with variables
    Input(ExprAST),
    Drop(ExprAST),
}
