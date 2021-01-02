use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum YeeError {
    ParseFieldError { field_name: &'static str, source: Option<ParseIntError> },
    FieldNotFound { field_name: &'static str },
}

impl Display for YeeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", match self {
            YeeError::ParseFieldError { .. } => "ParseFieldError",
            YeeError::FieldNotFound { .. } => "FieldNotFound"
        }, match self {
            YeeError::ParseFieldError { field_name, .. } => format!("failed to parse required field: {}", field_name),
            YeeError::FieldNotFound { field_name } => format!("did not find the required field: {}", field_name)
        })
    }
}

impl Error for YeeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YeeError::ParseFieldError { source, .. } => match *source {
                Some(ref x) => Some(x),
                None => None
            },
            YeeError::FieldNotFound { .. } => None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
