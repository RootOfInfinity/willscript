use std::{collections::HashMap, io};

use crate::{
    ast::{Assignment, BuiltIn, ExprAST, FunctionAST, IfBlock, Statement, WhileBlock},
    lexer::Operator,
};

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
        let mut varmap: HashMap<String, i32> = HashMap::new();
        let func = self
            .funcmap
            .get(func_name)
            .expect("Called unknown function.")
            .clone();
        for (i, arg) in func.proto.args.iter().enumerate() {
            varmap.insert(arg.clone(), args[i]);
        }
        for statement in &func.body {
            if let Some(x) = self.run_statement(statement, &mut varmap) {
                return x;
            }
        }
        todo!()
    }
    fn run_statement(
        &mut self,
        statement: &Statement,
        varmap: &mut HashMap<String, i32>,
    ) -> Option<i32> {
        match statement {
            Statement::Assign(x) => self.run_assignment(x, varmap),
            Statement::Call(x) => {
                self.eval_expr(x, varmap);
            }
            Statement::If(x) => {
                if let Some(x) = self.run_if_block(x, varmap) {
                    return Some(x);
                }
            }
            Statement::While(x) => {
                if let Some(x) = self.run_while_block(x, varmap) {
                    return Some(x);
                }
            }
            Statement::Built(BuiltIn::Return(x)) => return Some(self.eval_expr(x, varmap)),
            Statement::Built(x) => self.run_built(x, varmap),
        }
        None
    }
    fn run_assignment(&mut self, assignment: &Assignment, varmap: &mut HashMap<String, i32>) {
        let rhs = self.eval_expr(&assignment.right_hand, varmap);
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
    fn run_if_block(
        &mut self,
        if_block: &IfBlock,
        varmap: &mut HashMap<String, i32>,
    ) -> Option<i32> {
        if self.eval_expr(&if_block.conditional, varmap) != 0 {
            for statement in if_block.body.iter() {
                if let Some(x) = self.run_statement(&statement, varmap) {
                    return Some(x);
                }
            }
        }
        None
    }
    fn run_while_block(
        &mut self,
        while_block: &WhileBlock,
        varmap: &mut HashMap<String, i32>,
    ) -> Option<i32> {
        while self.eval_expr(&while_block.conditional, varmap) != 0 {
            for statement in while_block.body.iter() {
                if let Some(x) = self.run_statement(&statement, varmap) {
                    return Some(x);
                }
            }
        }
        None
    }
    fn run_built(&mut self, built: &BuiltIn, varmap: &mut HashMap<String, i32>) {
        match built {
            BuiltIn::Print(x) => println!("{}", self.eval_expr(x, varmap)),
            BuiltIn::Input(x) => {
                let ExprAST::Variable(name) = x else {
                    unreachable!();
                };
                let mut buf = String::new();
                io::stdin()
                    .read_line(&mut buf)
                    .expect("could not get stdin");
                let num = buf.parse::<i32>().expect("not a number");
                varmap.insert(name.clone(), num);
            }
            BuiltIn::Drop(x) => {
                let ExprAST::Variable(name) = x else {
                    unreachable!();
                };
                varmap.remove(name);
            }
            BuiltIn::Return(_) => unreachable!(),
        }
    }
    fn eval_expr(&mut self, binop: &ExprAST, varmap: &mut HashMap<String, i32>) -> i32 {
        match binop {
            ExprAST::Variable(x) => *varmap.get(x).expect("Could not find variable"),
            ExprAST::Number(x) => *x,
            ExprAST::Call(name, exprvec) => {
                let mut argvec = Vec::new();
                for arg in exprvec {
                    argvec.push(self.eval_expr(arg, varmap));
                }
                self.run_function(name, argvec)
            }
            ExprAST::BinOp(op, lhs, rhs) => {
                let lhs = self.eval_expr(lhs, varmap);
                let rhs = self.eval_expr(rhs, varmap);
                match op {
                    Operator::And => {
                        if lhs != 0 && rhs != 0 {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Or => {
                        if lhs != 0 || rhs != 0 {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Xor => {
                        if (lhs != 0 && rhs == 0) || (lhs == 0 && rhs != 0) {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::LEq => {
                        if lhs <= rhs {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::GEq => {
                        if lhs >= rhs {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Eq => {
                        if lhs == rhs {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Ls => {
                        if lhs < rhs {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::Gr => {
                        if lhs > rhs {
                            1
                        } else {
                            0
                        }
                    }
                    Operator::BAnd => lhs & rhs,
                    Operator::BOr => lhs | rhs,
                    Operator::BXor => lhs ^ rhs,
                    Operator::Add => lhs + rhs,
                    Operator::Sub => lhs - rhs,
                    Operator::Mult => lhs * rhs,
                }
            }
        }
    }
}
