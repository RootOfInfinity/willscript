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
        match self.cur_tok.clone() {
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
