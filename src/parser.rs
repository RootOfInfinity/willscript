use crate::{
    ast::ExprAST,
    lexer::{Operator, Token},
};
use std::vec::IntoIter;

struct ParsingMachine {
    cur_tok: Token,
    tok_iter: IntoIter<Token>,
}
impl ParsingMachine {
    pub fn new(cur_tok: Token, tok_iter: IntoIter<Token>) -> Self {
        ParsingMachine { cur_tok, tok_iter }
    }
    fn eat_tok(&mut self) {
        self.cur_tok = match self.tok_iter.next() {
            Some(x) => x,
            None => Token::EndOfFile,
        }
    }
    fn parse_expr(&mut self) -> Result<ExprAST, String> {
        todo!()
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
        todo!()
    }
    fn get_priority(operator: &Operator) -> u32 {
        match operator {
            Operator::LEq | Operator::Ls | Operator::GEq | Operator::Gr | Operator::Eq => 10,
            Operator::And | Operator::Or | Operator::Xor => 20,
            Operator::BAnd | Operator::BOr | Operator::BXor => 30,
            Operator::Add | Operator::Sub => 40,
            Operator::Mult => 50,
        }
    }
}
