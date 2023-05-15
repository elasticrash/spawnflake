use std::fmt::Display;

use crate::datastores::generic::common_models::TableFields;
#[derive(Debug, Clone)]
pub struct Mysql {
    pub schema: Vec<TableFields>,
}

impl Display for Mysql {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in &self.schema {
            writeln!(f, "{}", x.table_name)?
        }

        Ok(())
    }
}
