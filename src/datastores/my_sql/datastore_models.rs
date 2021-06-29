use crate::datastores::generic::common_models::{TableFields};
#[derive(Debug, Clone)]
pub struct Mysql {
    pub schema: Vec<TableFields>,
}