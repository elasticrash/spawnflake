#[derive(Debug, Clone)]
pub struct ForeignKeyRel {
    pub table_name: String,
    pub column_name: String,
    pub referenced_table_name: String,
    pub referenced_column_name: String,
}

#[derive(Debug, Clone)]
pub struct TableFields {
    pub table_name: String,
    pub fields: Vec<Describe>,
    pub rel: Vec<ForeignKeyRel>,
}


#[derive(Debug, Clone)]
pub struct Describe {
    pub field: String,
    pub data_type: String,
    pub null: String,
    pub key: String,
    pub default: Option<String>,
    pub extra: String,
}
