use std::str::Chars;

pub struct LexingMachine<'a> {
    cur_char: char,
    chars: Chars<'a>,
    lexing_finished: bool,
}
impl<'a> LexingMachine<'a> {
    pub fn new(cur_char: char, chars: Chars<'a>) -> Self {
        LexingMachine {
            cur_char,
            chars,
            lexing_finished: false,
        }
    }
    pub fn activate_lexing(&mut self) -> Vec<Token> {
        let mut tokvec = Vec::new();
        loop {
            let tok = self.get_token();
            tokvec.push(tok.clone());
            if let Token::EndOfFile = tok {
                break;
            }
        }
        tokvec
    }
    fn eat_char(&mut self) {
        self.cur_char = match self.chars.next() {
            Some(x) => x,
            None => {
                self.lexing_finished = true;
                ' '
            }
        }
    }
    fn cur_is_alpha(&self, no_nums: bool) -> bool {
        if (self.cur_char >= 'a' && self.cur_char <= 'z')
            || (self.cur_char >= 'A' && self.cur_char <= 'Z')
        {
            true
        } else if !no_nums && self.cur_char >= '0' && self.cur_char <= '9' {
            true
        } else {
            false
        }
    }
    fn cur_is_digit(&self) -> bool {
        if self.cur_char >= '0' && self.cur_char <= '9' {
            true
        } else {
            false
        }
    }
    fn cur_is_op(&self) -> bool {
        match self.cur_char {
            '<' => true,
            '>' => true,
            '&' => true,
            '=' => true,
            '|' => true,
            '^' => true,
            '+' => true,
            '-' => true,
            '*' => true,
            _ => false,
        }
    }
    fn get_token(&mut self) -> Token {
        if self.lexing_finished {
            return Token::EndOfFile;
        }
        while self.cur_char.is_ascii_whitespace() {
            if self.lexing_finished {
                return Token::EndOfFile;
            }
            self.eat_char();
        }
        //Whitespace done
        if self.cur_is_alpha(true) {
            let mut ident_str = String::new();
            ident_str.push(self.cur_char);
            self.eat_char();
            while self.cur_is_alpha(false) {
                ident_str.push(self.cur_char);
                self.eat_char();
            }
            return match ident_str.as_str() {
                "var" => Token::Var,
                "if" => Token::If,
                "fun" => Token::Fun,
                "return" => Token::Return,
                "print" => Token::Print,
                "input" => Token::Input,
                "drop" => Token::Drop,
                x => Token::Identifier(x.to_owned()),
            };
        }
        //And thats Identifiers done!
        if self.cur_is_digit() {
            let mut dig_string = String::new();
            dig_string.push(self.cur_char);
            self.eat_char();
            while self.cur_is_digit() {
                dig_string.push(self.cur_char);
                self.eat_char();
            }
            let num: i32 = dig_string
                .parse()
                .expect("Critical error in parsing number");
            return Token::Number(num);
        }
        //Numbers done!
        if self.cur_is_op() {
            // big bundle of if statements
            // I don't know how else to handle finite atomata
            if self.cur_char == '<' {
                self.eat_char();
                if self.cur_is_op() {
                    if self.cur_char == '=' {
                        return Token::Op(Operator::LEq);
                    } else {
                        panic!("Lexing Error, bad operator");
                    }
                } else {
                    return Token::Op(Operator::Ls);
                }
            } else if self.cur_char == '>' {
                self.eat_char();
                if self.cur_is_op() {
                    if self.cur_char == '=' {
                        return Token::Op(Operator::GEq);
                    } else {
                        panic!("Lexing Error, bad operator");
                    }
                } else {
                    return Token::Op(Operator::Gr);
                }
            } else if self.cur_char == '=' {
                self.eat_char();
                if self.cur_is_op() {
                    if self.cur_char == '=' {
                        return Token::Op(Operator::Eq);
                    } else {
                        panic!("Lexing Error, bad operator");
                    }
                } else {
                    return Token::Assignment;
                }
            } else if self.cur_char == '&' {
                self.eat_char();
                if self.cur_char == '&' {
                    return Token::Op(Operator::And);
                } else if self.cur_is_op() {
                    panic!("Lexing Error, bad operator");
                } else {
                    return Token::Op(Operator::BAnd);
                }
            } else if self.cur_char == '|' {
                self.eat_char();
                if self.cur_char == '|' {
                    return Token::Op(Operator::Or);
                } else if self.cur_is_op() {
                    panic!("Lexing Error, bad operator");
                } else {
                    return Token::Op(Operator::BOr);
                }
            } else if self.cur_char == '^' {
                self.eat_char();
                if self.cur_char == '^' {
                    return Token::Op(Operator::Xor);
                } else if self.cur_is_op() {
                    panic!("Lexing Error, bad operator");
                } else {
                    return Token::Op(Operator::BXor);
                }
            } else if self.cur_char == '+' {
                self.eat_char();
                return Token::Op(Operator::Add);
            } else if self.cur_char == '-' {
                self.eat_char();
                return Token::Op(Operator::Sub);
            } else if self.cur_char == '*' {
                self.eat_char();
                return Token::Op(Operator::Mult);
            } else {
                unreachable!();
            }
        }
        //Operators finally done... lots of boilerplate.
        // I might be able to fix it with some match statements.
        let this_char = self.cur_char;
        self.eat_char();
        return match this_char {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftCurly,
            '}' => Token::RightCurly,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            _ => panic!("unexpected char (are you only using ASCII?)"),
        };
        //Nice clean ending, with all the other chars.
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    // Add more when the time comes
    Identifier(String),
    Number(i32),
    Var,
    Fun,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    Op(Operator),
    Return,
    If,
    Assignment,
    Semicolon,
    Comma,
    EndOfFile,
    Print,
    Input,
    Drop,
}

#[derive(Clone, Debug)]
pub enum Operator {
    //Logical
    And,
    Or,
    Xor,
    //Comparison
    LEq,
    GEq,
    Eq,
    Ls,
    Gr,
    //Bitwise
    BAnd,
    BOr,
    BXor,
    //Math (Multiply is given higher priority)
    Add,
    Sub,
    Mult,
}
