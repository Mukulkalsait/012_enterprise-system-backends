// error.rs

use crate::book::*;
use crate::member::*;

#[derive(Debug, Clone)]
pub enum LibBookErrors {
    NotFound,
    Unavialable { message: String, id: Option<MemberId> },
    InvalidResponse { messgage: String, expected: Option<String>, found: Option<String> },
    TimeOut { message: String, duration: Option<chrono::Duration> },
}
impl std::fmt::Display for LibBookErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibBookErrors::NotFound => {
                write!(f, "Book not found in the Library")
            },
            LibBookErrors::Unavialable { message, id } => {
                if let Some(id) = id {
                    write!(f, "Book is borrowed by user:{}", id)
                } else {
                    write!(f, "{}", message)
                }
            },
            LibBookErrors::InvalidResponse { messgage, expected, found } => {
                if let (Some(expected), Some(found)) = (expected, found) {
                    write!(f, "message:{}, expected: {}, Recieved:{} ", messgage, expected, found)
                } else {
                    write!(f, "message:{}", messgage)
                }
            },
            LibBookErrors::TimeOut { message, duration } => {
                if let Some(duration) = duration {
                    write!(f, "Timeout:{} Timelimit: {:?}", message, duration)
                } else {
                    write!(f, "Timeout:{}", message)
                }
            },
        }
    }
}

impl std::error::Error for LibBookErrors {}

#[derive(Debug, Clone)]
pub enum LibUserErrors {
    NotFound,
    Error { message: String },
}

impl std::fmt::Display for LibUserErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LibUserErrors::NotFound => {
                write!(f, "User not found")
            },
            LibUserErrors::Error { message } => {
                write!(f, "Error: {}", message)
            },
        }
    }
}

impl std::error::Error for LibUserErrors {}
