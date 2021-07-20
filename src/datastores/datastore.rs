use super::generic::common_models::{ForeignKeyRel, TableFields};
use crate::configuration::config_model::GenericConfiguration;
use std::collections::VecDeque;

pub trait DataGeneration<T> {
    fn spawn(&mut self, config: &GenericConfiguration, no_of_record: i32);
    fn set_schema(&mut self, conn: &mut T, schema: &String);
    fn new() -> Self;
    fn build_depedency_tree(
        &mut self,
        safe_tf: &mut VecDeque<TableFields>,
        unsafe_tf: &mut VecDeque<TableFields>,
        cyclic_dependency_check: bool
    ) -> (VecDeque<TableFields>, VecDeque<TableFields>);
}
