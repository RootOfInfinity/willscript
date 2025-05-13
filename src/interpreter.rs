use std::collections::HashMap;

use crate::ast::{Assignment, BuiltIn, ExprAST, FunctionAST, IfBlock, Statement};

pub struct InterpretingMastermind {
    funcmap: HashMap<String, FunctionAST>,
}
impl InterpretingMastermind {
    pub fn new(mut funcvec: Vec<FunctionAST>) -> Self {
        funcvec = funcvec
            .into_iter()
            .map(|mut x| {
                x.body
                    .push(Statement::Built(BuiltIn::Return(ExprAST::Number(0))));
                x
            })
            .collect();
        // that is temporary until we analyise the code and add the void type.
        let mut funcmap = HashMap::with_capacity(funcvec.len());
        for func in funcvec {
            funcmap.insert(func.proto.name.clone(), func);
        }
        InterpretingMastermind { funcmap }
    }
    pub fn run_main(&mut self) {
        self.run_function(&"main".to_string(), vec![]);
    }
    fn run_function(&mut self, func_name: &String, args: Vec<i32>) -> i32 {
        let mut varmap = HashMap::new();
        let func = self
            .funcmap
            .get(func_name)
            .expect("Called unknown function.")
            .clone();
        for statement in &func.body {
            match statement {
                Statement::Assign(x) => self.run_assignment(x, &mut varmap),
                Statement::If(x) => self.run_if_block(x, &mut varmap),
                Statement::Call(x) => self.run_call(x, &mut varmap),
                Statement::Built(x) => self.run_built(x, &mut varmap),
            }
        }
        todo!()
    }
    fn run_assignment(&mut self, assignment: &Assignment, varmap: &mut HashMap<String, i32>) {
        let rhs = self.eval_binop(&assignment.right_hand, varmap);
        let ExprAST::Variable(ref varname) = assignment.variable else {
            eprintln!("The parser messed up, and this exprast is wrong");
            panic!();
        };
        if assignment.is_declaration {
            if varmap.contains_key(varname) {
                panic!("Declared item, but we already have the key!");
            }
            varmap.insert(varname.clone(), rhs);
        } else {
            if !varmap.contains_key(varname) {
                panic!("Tried to assign item, but it wasn't declared!");
            }
            varmap.insert(varname.clone(), rhs);
        }
    }
    fn run_if_block(&mut self, if_block: &IfBlock, varmap: &mut HashMap<String, i32>) {
        todo!()
    }
    fn run_call(&mut self, call: &ExprAST, varmap: &mut HashMap<String, i32>) {
        todo!()
    }
    fn run_built(&mut self, built: &BuiltIn, varmap: &mut HashMap<String, i32>) {
        todo!()
    }
    fn eval_binop(&mut self, binop: &ExprAST, varmap: &mut HashMap<String, i32>) -> i32 {
        todo!()
    }
}
