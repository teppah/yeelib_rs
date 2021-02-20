//! fsadqqqqqqqqqq
//!
//!
//!
//! ssssssssssssasdfasdf
//! asdfsad
//! # examples
//! asdfjklasdf
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// this is a req
#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    pub id: u16,
    pub method: String,
    pub params: Vec<Value>,
}

impl Req {
    pub fn with_id(id: u16, method: String, params: Vec<Value>) -> Req {
        Req { id, method, params }
    }
    pub fn new(method: String, params: Vec<Value>) -> Req {
        let id = fastrand::u16(..);
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