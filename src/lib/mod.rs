pub mod context;
pub mod error;

pub struct Program {
    source: String,
    stack: Vec<context::Context>,
}

impl Program {
    pub fn new(filename: String, source: String) -> Self {
        Self {
            source,
            stack: vec![context::Context::new(format!("file:{0}", filename), None)],
        }
    }

    pub fn run(&mut self) -> Result<(), error::Error> {
        self.stack[0].evaluate();
        
        Ok(())
    }
}
