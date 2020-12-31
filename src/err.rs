use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum YeeError {
    ParseFieldError(String),
}

impl Display for YeeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", match self {
            YeeError::ParseFieldError(_) => "ParseFieldError"
        }, match self {
            YeeError::ParseFieldError(s) => s
        })
    }
}

impl Error for YeeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YeeError::ParseFieldError(s) => None,
        }
    }
}

impl From<num::ParseIntError> for YeeError {
    fn from(e: ParseIntError) -> Self {
        YeeError::ParseFieldError(format!("Failed conversion to a number: {}", e))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
