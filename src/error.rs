use std::fmt::Display;

pub type Result<R, E = Error> = std::result::Result<R, E>;

#[derive(Debug)]
pub struct Error(pub String);

impl<T: Display + Send + Sync> From<T> for Error {
    fn from(err: T) -> Self {
        Self(err.to_string())
    }
}
