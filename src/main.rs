use std::fs;

use lexer::LexingMachine;
use parser::ParsingMachine;

mod ast;
mod lexer;
mod parser;

fn main() {
    let raw_string = fs::read_to_string("./willcode.ws").expect("Could not find file");
    let mut file_iter = raw_string.chars();
    let cur_char = file_iter
        .next()
        .expect("Come on, you gotta have at least one character, right?");
    let mut awesome_lexing_machine = LexingMachine::new(cur_char, file_iter);
    let tokvec = awesome_lexing_machine.activate_lexing();
    for val in tokvec.iter() {
        println!("{:#?}", val);
    }
    let mut tok_iter = tokvec.into_iter().peekable();
    let cur_tok = tok_iter
        .next()
        .expect("Come on, you gotta have at least one token, right?");
    let mut amazing_parsing_machine = ParsingMachine::new(cur_tok, tok_iter);
    let ast_vec = match amazing_parsing_machine.activate_parsing_machine() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            panic!()
        }
    };
    for val in ast_vec {
        println!("{:#?}", val);
    }
}
