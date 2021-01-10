use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum YeeError {
    ParseFieldFailed { field_name: &'static str, source: Option<ParseIntError> },
    FieldNotFound { field_name: &'static str },
    IoError { source: std::io::Error },
    MethodNotSupported { method_name: &'static str },
    InvalidValue { field_name: &'static str, value: String },
    ChangeFailed { message: String },
}

impl Display for YeeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", match self {
            YeeError::ParseFieldFailed { .. } => "ParseFieldFailed",
            YeeError::FieldNotFound { .. } => "FieldNotFound",
            YeeError::IoError { .. } => "IoError",
            YeeError::MethodNotSupported { .. } => "MethodNotSupported",
            YeeError::InvalidValue { .. } => "InvalidValue",
            YeeError::ChangeFailed { .. } => "ChangeFailed"
        }, match self {
            YeeError::ParseFieldFailed { field_name, .. } => format!("failed to parse required field: {}", field_name),
            YeeError::FieldNotFound { field_name } => format!("did not find the required field: {}", field_name),
            YeeError::IoError { source } => format!("IO error: {}", source),
            YeeError::MethodNotSupported { method_name } => format!("cannot use method: {}", method_name),
            YeeError::InvalidValue { field_name, value } => format!("invalid value for {}: {}", field_name, value),
            YeeError::ChangeFailed { message } => format!("changing param failed: {}", message)
        })
    }
}

impl Error for YeeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YeeError::ParseFieldFailed { source, .. } => source.as_ref().map(|v| v as _),
            YeeError::IoError { source } => Some(source),
            _ => None
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
