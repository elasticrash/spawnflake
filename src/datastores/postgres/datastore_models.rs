#[derive(Debug, Clone)]
pub struct TableFields {
    pub table_name: String,
}

#[derive(Debug, Clone)]
pub struct Postgres {
    pub schema: Vec<TableFields>,
}