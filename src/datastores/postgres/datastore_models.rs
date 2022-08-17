use crate::datastores::generic::common_models::TableFields;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub schema: Vec<TableFields>,
}