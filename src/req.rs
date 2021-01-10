use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    id: i32,
    method: String,
    params: Vec<Value>,
}

impl Req {
    pub fn new(id: i32, method: String, params: Vec<Value>) -> Req {
        Req { id, method, params }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Transition {
    Sudden,
    Smooth {
        // minimum 30ms
        duration: Duration
    },
}

impl Transition {
    pub fn sudden() -> Transition {
        Self::Sudden
    }
    pub fn smooth(duration: Duration) -> Option<Transition> {
        if duration < Duration::from_millis(30) || duration.as_millis() > u64::MAX as u128 {
            None
        } else {
            Some(Self::Smooth { duration })
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            Self::Sudden => "sudden",
            Self::Smooth { .. } => "smooth"
        }
    }

    pub fn value(&self) -> u64 {
        match self {
            // is ignored anyway
            Self::Sudden => 0,
            Self::Smooth { duration } => duration.as_millis() as u64
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}