use serde::{Serialize, Deserialize};
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

#[cfg(test)]
mod tests {
    use crate::req::Req;

    #[test]
    fn test() {}
}