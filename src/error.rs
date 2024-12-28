use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(String),
}

// 实现 fmt::Display trait
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "I/O error: {}", e),
        }
    }
}