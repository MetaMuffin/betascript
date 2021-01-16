
#[derive(Debug)]
pub enum Error {
    ReturnError(String),
    SyntaxError(String),
    NotImplementedError(String),
}
