#[derive(Debug)]
pub struct Chain<'a> {
    pub tokens: Vec<&'a str>,
    pub rivet: Box<Option<&'a Chain<'a>>>,
}

impl<'a> Chain<'a> {
    pub fn new(tokens: std::vec::Vec<&'static str>) -> Chain<'static> {
        Chain {
            tokens: tokens,
            rivet: Box::new(None),
        }
    }
    pub fn set(&mut self, r: &'a Chain) {
        self.rivet = Box::new(Some(r));
    }
}
