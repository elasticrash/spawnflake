use crate::datastores::generic::common_models::{ForeignKeyRel, TableFields};
#[derive(Debug, Clone)]
pub struct Mysql {
    pub schema: Vec<TableFields>,
}

#[derive(Debug, Clone)]
pub struct CdDt {
    pub name: String,
    pub data_type: String,
    pub fk: bool,
    pub dep: Option<ForeignKeyRel>,
}

#[derive(Debug, Clone)]
pub struct TempKeys {
    pub id: Vec<i32>,
    pub table_name: String,
}
