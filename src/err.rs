use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum YeeError {
    ParseFieldError { field_name: &'static str, source: Option<ParseIntError> },
    FieldNotFound { field_name: &'static str },
    IoError { source: std::io::Error },
}

impl Display for YeeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", match self {
            YeeError::ParseFieldError { .. } => "ParseFieldError",
            YeeError::FieldNotFound { .. } => "FieldNotFound",
            YeeError::IoError { .. } => "IoError"
        }, match self {
            YeeError::ParseFieldError { field_name, .. } => format!("failed to parse required field: {}", field_name),
            YeeError::FieldNotFound { field_name } => format!("did not find the required field: {}", field_name),
            YeeError::IoError { source } => format!("IO error: {}", source)
        })
    }
}

impl Error for YeeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YeeError::ParseFieldError { source, .. } => source.as_ref().map(|v| v as _),
            YeeError::FieldNotFound { .. } => None,
            YeeError::IoError { source } => Some(source)
        }
    }
}

impl From<std::io::Error> for YeeError {
    fn from(e: std::io::Error) -> Self {
        YeeError::IoError { source: e }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
