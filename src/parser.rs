use crate::lexer::Token;

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
    fn parse_ident(&mut self) -> Result<ExprAST, String> {
        let Token::Identifier(ident_string) = self.cur_tok else {
            return Err("Parse Ident did not get an identifier.".to_owned());
        };
        self.eat_tok();
        Ok(ExprAST::Variable(ident_string))
    }
    fn parse_num(&mut self) -> Result<ExprAST, String> {
        let Token::Number(num) = self.cur_tok else {
            return Err("Parse Num did not get a number.".to_owned());
        };
        self.eat_tok();
        Ok(ExprAST::Number(num));
    }
}
