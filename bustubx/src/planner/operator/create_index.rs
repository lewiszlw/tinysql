use crate::catalog::{Schema, SchemaRef};

#[derive(derive_new::new, Debug, Clone)]
pub struct LogicalCreateIndexOperator {
    pub index_name: String,
    pub table_name: String,
    pub table_schema: SchemaRef,
    pub key_attrs: Vec<u32>,
}
