use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    id: i32,
    method: String,
    params: Vec<String>,
}

impl Req {
    pub fn new(id: i32, method: String, params: Vec<String>) -> Req {
        Req { id, method, params }
    }
}

#[cfg(test)]
mod tests {
    use crate::req::Req;

    #[test]
    fn test() {}
}