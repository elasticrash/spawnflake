#[derive(Debug, Clone)]
pub struct Chain {
    pub tokens: Vec<String>,
}

/// constructor for a string chain
impl Chain {
    pub fn new(tokens: std::vec::Vec<String>) -> Chain {
        Chain { tokens }
    }
}
