use std::fs;

use lexer::LexingMachine;

mod lexer;

fn main() {
    let raw_string = fs::read_to_string("./willcode.ws").expect("Could not find file");
    let mut file_iter = raw_string.chars();
    let cur_char = file_iter
        .next()
        .expect("Come on, you gotta have at least one character, right?");
    let mut awesome_lexing_machine = LexingMachine::new(cur_char, file_iter);
    let tokvec = awesome_lexing_machine.activate_lexing();
    for val in tokvec {
        println!("{:#?}", val);
    }
}
