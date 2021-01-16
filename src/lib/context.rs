use super::Program;
use std::collections::HashMap;
pub struct Context {
    name: String,
    pc: usize,
    return_pc: Option<usize>,
    variables: HashMap<String, Value>,
}

pub enum Value {
    Undefined,
    Number(i64),
    Function(usize),
}

impl Context {
    pub fn new(name: String, pc: usize, return_pc: Option<usize>) -> Context {
        Context {
            pc,
            name,
            return_pc,
            variables: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, prog: &mut Program) -> Value {
        let mut end_return = Value::Undefined;
        let mut start_code = match prog.source.find(",") {
            Some(u) => u,
            None => self.pc,
        };

        return end_return;
    }
}
