use crate::{
    ast::{Assignment, BuiltIn, ExprAST, FunctionAST, IfBlock, PrototypeAST, Statement},
    lexer::{Operator, Token},
};
use std::{iter::Peekable, vec::IntoIter};

pub struct ParsingMachine {
    cur_tok: Token,
    tok_iter: Peekable<IntoIter<Token>>,
}
impl ParsingMachine {
    pub fn new(cur_tok: Token, tok_iter: Peekable<IntoIter<Token>>) -> Self {
        ParsingMachine { cur_tok, tok_iter }
    }
    fn eat_tok(&mut self) {
        self.cur_tok = match self.tok_iter.next() {
            Some(x) => x,
            None => Token::EndOfFile,
        }
    }
    pub fn activate_parsing_machine(&mut self) -> Result<Vec<FunctionAST>, String> {
        let mut ansvec = Vec::new();
        loop {
            match &self.cur_tok {
                Token::Fun => {
                    let fun = self.parse_function()?;
                    ansvec.push(fun);
                }
                Token::EndOfFile => break,
                x => return Err(format!("Expected 'fun' or EOF. Got: {:#?}", x)),
            }
        }
        Ok(ansvec)
    }
    fn parse_function(&mut self) -> Result<FunctionAST, String> {
        let Token::Fun = self.cur_tok else {
            return Err("Parse function called, but it didn't even start with 'fun'.".to_owned());
        };
        let proto = self.parse_proto()?;
        let body = self.collect_statements()?;
        Ok(FunctionAST::new(proto, body))
    }
    fn parse_proto(&mut self) -> Result<PrototypeAST, String> {
        self.eat_tok(); // eats the 'fun'
        // lol funny comment ^
        let Token::Identifier(name) = self.cur_tok.clone() else {
            return Err("The prototype needs a name bro.".to_owned());
        };
        self.eat_tok(); // eats the name
        let Token::LeftParen = self.cur_tok else {
            return Err("Every prototype needs a left parenthesis.".to_owned());
        };
        self.eat_tok(); // eats the left parenthesis
        let mut args = Vec::new();
        while !matches!(self.cur_tok, Token::RightParen) {
            let Token::Identifier(arg_name) = self.cur_tok.clone() else {
                return Err("Not an ident inside prototype.".to_owned());
            };
            self.eat_tok();
            args.push(arg_name);
            match &self.cur_tok {
                Token::Comma => {
                    self.eat_tok();
                    continue;
                }
                Token::RightParen => break,
                x => return Err(format!("Unexpected token in prototype: {:#?}", x)),
            }
        }
        self.eat_tok(); // eat the right parenthesis
        Ok(PrototypeAST::new(name, args))
    }
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.cur_tok {
            Token::Var => Ok(Statement::Assign(self.parse_assignment()?)),
            Token::Identifier(_) => match self.tok_iter.peek() {
                Some(&Token::LeftParen) => self.parse_call(),
                _ => Ok(Statement::Assign(self.parse_assignment()?)),
            },
            Token::If => Ok(Statement::If(self.parse_ifblock()?)),
            Token::Print | Token::Input | Token::Drop | Token::Return => {
                Ok(Statement::Built(self.parse_builtin()?))
            }
            x => Err(format!("Expected the start of a statement, got: {:#?}", x)),
        }
    }
    fn collect_statements(&mut self) -> Result<Vec<Statement>, String> {
        self.eat_tok(); // eats open curly brace
        let mut codevec = Vec::new();
        while !matches!(self.cur_tok, Token::RightCurly) {
            let statement = self.parse_statement()?;
            codevec.push(statement);
        }
        // it matches a right curly, so eat that.
        self.eat_tok();
        Ok(codevec)
    }
    fn parse_builtin(&mut self) -> Result<BuiltIn, String> {
        match &self.cur_tok {
            Token::Print => {
                self.eat_tok(); // eat the print
                let expr = self.parse_expr()?;
                let Token::Semicolon = self.cur_tok else {
                    return Err("No semicolon after print statement.".to_owned());
                };
                self.eat_tok(); // eat the semicolon
                Ok(BuiltIn::Print(expr))
            }
            Token::Return => {
                self.eat_tok(); // eat the print
                let expr = self.parse_expr()?;
                let Token::Semicolon = self.cur_tok else {
                    return Err("No semicolon after return statement.".to_owned());
                };
                self.eat_tok(); // eat the semicolon
                Ok(BuiltIn::Return(expr))
            }
            Token::Input => {
                self.eat_tok(); // eat the input
                let expr = self.parse_expr()?;
                let ExprAST::Variable(_) = expr else {
                    return Err("Input did not recieve a variable.".to_owned());
                };
                let Token::Semicolon = self.cur_tok else {
                    return Err("No semicolon after input statement".to_owned());
                };
                self.eat_tok(); // eat the semicolon
                Ok(BuiltIn::Input(expr))
            }
            Token::Drop => {
                self.eat_tok(); // eat the drop
                let expr = self.parse_expr()?;
                let ExprAST::Variable(_) = expr else {
                    return Err("Drop did not recieve a variable.".to_owned());
                };
                let Token::Semicolon = self.cur_tok else {
                    return Err("No semicolon after drop statement".to_owned());
                };
                self.eat_tok(); // eat the semicolon
                Ok(BuiltIn::Drop(expr))
            }
            x => Err(format!("Expected print, input, or drop, got: {:#?}", x)),
        }
    }
    fn parse_ifblock(&mut self) -> Result<IfBlock, String> {
        let Token::If = self.cur_tok else {
            return Err("Could not find 'if'".to_owned());
        };
        self.eat_tok(); //eat the 'if'
        let conditional = self.parse_expr()?;
        let Token::LeftCurly = self.cur_tok else {
            return Err("Could not find '{' required for if block.".to_owned());
        };
        let statements = self.collect_statements()?;
        // we dont need to check for right curly, collect statements already does that.
        Ok(IfBlock::new(conditional, statements))
    }
    fn parse_call(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expr()?;
        let ExprAST::Call(_, _) = expr else {
            return Err("Not a call. Think long and hard about that one.".to_owned());
        };
        Ok(Statement::Call(expr))
    }
    fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let is_declaration = match self.cur_tok {
            Token::Var => {
                self.eat_tok();
                true
            }
            _ => false,
        };
        let ExprAST::Variable(var_string) = self.parse_expr()? else {
            return Err(
                "No ident after var. (or you put parse_assignment the wrong place)".to_owned(),
            );
        };
        let Token::Assignment = self.cur_tok else {
            return Err("No assignment after variable.".to_owned());
        };
        self.eat_tok();
        let expr = self.parse_expr()?;
        match &self.cur_tok {
            Token::Semicolon => {
                self.eat_tok(); // eats the semicolon
                Ok(Assignment::new(
                    is_declaration,
                    ExprAST::Variable(var_string),
                    expr,
                ))
            }
            x => Err(format!("Expected semicolon, got {:#?}.", x)),
        }
    }
    fn parse_expr(&mut self) -> Result<ExprAST, String> {
        let lhs = self.parse_primary()?;

        self.parse_rhs(0, lhs)
    }
    fn parse_rhs(&mut self, expr_prior: u32, lhs: ExprAST) -> Result<ExprAST, String> {
        let mut lhs = lhs;
        loop {
            let Token::Op(binop) = self.cur_tok.clone() else {
                return Ok(lhs);
            };
            let tok_prior = get_priority(&binop);
            if tok_prior < expr_prior {
                return Ok(lhs);
            }
            self.eat_tok(); // eating the operator
            let mut rhs = self.parse_primary()?;
            if let Token::Op(new_binop) = self.cur_tok.clone() {
                if get_priority(&new_binop) > tok_prior {
                    rhs = self.parse_rhs(tok_prior + 1, rhs)?;
                }
            }
            lhs = ExprAST::BinOp(binop, Box::new(lhs), Box::new(rhs));
        }
    }
    fn parse_ident(&mut self) -> Result<ExprAST, String> {
        let Token::Identifier(ident_string) = self.cur_tok.clone() else {
            return Err("Parse Ident did not get an identifier.".to_owned());
        };
        self.eat_tok();
        if let Token::LeftParen = self.cur_tok {
            self.eat_tok();
            let mut arg_vec = Vec::new();
            loop {
                let expr = self.parse_expr()?;
                arg_vec.push(expr);
                match self.cur_tok {
                    Token::RightParen => break,
                    Token::Comma => self.eat_tok(), //eat the comma
                    _ => return Err("Unexpected token in function call".to_owned()),
                }
            }
            //eat right paren
            self.eat_tok();
            return Ok(ExprAST::Call(ident_string, arg_vec));
        }
        Ok(ExprAST::Variable(ident_string))
    }
    fn parse_num(&mut self) -> Result<ExprAST, String> {
        let Token::Number(num) = self.cur_tok else {
            return Err("Parse Num did not get a number.".to_owned());
        };
        self.eat_tok();
        Ok(ExprAST::Number(num))
    }
    fn parse_paren(&mut self) -> Result<ExprAST, String> {
        let Token::LeftParen = self.cur_tok else {
            return Err("No parentheses given to parse paren".to_owned());
        };
        // eat that left paren
        self.eat_tok();
        let expr = self.parse_expr()?;
        match self.cur_tok {
            Token::RightParen => Ok(expr),
            _ => Err("Expected right paren".to_owned()),
        }
    }
    fn parse_primary(&mut self) -> Result<ExprAST, String> {
        match &self.cur_tok {
            Token::Identifier(_) => self.parse_ident(),
            Token::Number(_) => self.parse_num(),
            Token::LeftParen => self.parse_paren(),
            x => Err(format!("Bad Token given to parse primary: {:#?}", x)),
        }
    }
}
fn get_priority(operator: &Operator) -> u32 {
    match operator {
        Operator::And | Operator::Or | Operator::Xor => 10,
        Operator::LEq | Operator::Ls | Operator::GEq | Operator::Gr | Operator::Eq => 20,
        Operator::BAnd | Operator::BOr | Operator::BXor => 30,
        Operator::Add | Operator::Sub => 40,
        Operator::Mult => 50,
    }
}
