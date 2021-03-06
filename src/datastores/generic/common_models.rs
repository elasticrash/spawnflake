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
pub struct NullableForeignKeys {
    pub column_name: String,
    pub nullable: bool,
    pub safe: bool,
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

#[derive(Debug, Clone)]
pub struct CdDt {
    pub name: String,
    pub data_type: String,
    pub fk: bool,
    pub non_generated: bool,
    pub dep: Option<ForeignKeyRel>,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct TempKeys {
    pub id: Vec<String>,
    pub table_name: String,
}
