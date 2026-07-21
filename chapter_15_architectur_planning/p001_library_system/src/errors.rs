// error.rs

use crate::member::*;

#[derive(Debug, Clone)]
pub enum LibErrors {
    NotFound,
    Unavialable { message: String, id: Option<MemberId> },
    InvalidResponse { messgage: String, expected: Option<String>, found: Option<String> },
    TimeOut { message: String, duration: Option<chrono::Duration> },
}
impl std::fmt::Display for LibErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibErrors::NotFound => {
                write!(f, "Book not found in the Library")
            }
            LibErrors::Unavialable { message, id } => {
                if let Some(id) = id {
                    write!(f, "Book is borrowed by user:{}", id)
                } else {
                    write!(f, "{}", message)
                }
            }
            LibErrors::InvalidResponse { messgage, expected, found } => {
                if let (Some(expected), Some(found)) = (expected, found) {
                    write!(f, "message:{}, expected: {}, Recieved:{} ", messgage, expected, found)
                } else {
                    write!(f, "message:{}", messgage)
                }
            }
            LibErrors::TimeOut { message, duration } => {
                if let Some(duration) = duration {
                    write!(f, "Timeout:{} Timelimit: {:?}", message, duration)
                } else {
                    write!(f, "Timeout:{}", message)
                }
            }
        }
    }
}

impl std::error::Error for LibErrors {}
pub type Result<T> = std::result::Result<T, LibErrors>;
