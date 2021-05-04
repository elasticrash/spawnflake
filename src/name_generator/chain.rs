#[derive(Debug, Clone)]
pub struct Chain {
    pub tokens: Vec<String>,
}

impl Chain {
    pub fn new(tokens: std::vec::Vec<String>) -> Chain {
        Chain { tokens: tokens }
    }
}
