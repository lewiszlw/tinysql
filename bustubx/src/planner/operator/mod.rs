use crate::catalog::{ColumnRef, SchemaRef};
use crate::{
    catalog::{catalog::TableOid, Schema},
    common::ScalarValue,
    planner::{expr::Expr, order_by::BoundOrderBy, table_ref::join::JoinType},
};

use self::{
    create_index::LogicalCreateIndexOperator, create_table::LogicalCreateTableOperator,
    filter::LogicalFilterOperator, insert::LogicalInsertOperator, join::LogicalJoinOperator,
    limit::LogicalLimitOperator, project::LogicalProjectOperator, scan::LogicalScanOperator,
    sort::LogicalSortOperator, values::LogicalValuesOperator,
};

pub mod create_index;
pub mod create_table;
pub mod filter;
pub mod insert;
pub mod join;
pub mod limit;
pub mod project;
pub mod scan;
pub mod sort;
pub mod values;

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    Dummy,
    CreateTable(LogicalCreateTableOperator),
    CreateIndex(LogicalCreateIndexOperator),
    // Aggregate(AggregateOperator),
    Filter(LogicalFilterOperator),
    Join(LogicalJoinOperator),
    Project(LogicalProjectOperator),
    Scan(LogicalScanOperator),
    Sort(LogicalSortOperator),
    Limit(LogicalLimitOperator),
    Insert(LogicalInsertOperator),
    Values(LogicalValuesOperator),
}
impl LogicalOperator {
    pub fn new_create_table_operator(table_name: String, schema: Schema) -> LogicalOperator {
        LogicalOperator::CreateTable(LogicalCreateTableOperator::new(table_name, schema))
    }
    pub fn new_create_index_operator(
        index_name: String,
        table_name: String,
        table_schema: SchemaRef,
        key_attrs: Vec<u32>,
    ) -> LogicalOperator {
        LogicalOperator::CreateIndex(LogicalCreateIndexOperator::new(
            index_name,
            table_name,
            table_schema,
            key_attrs,
        ))
    }
    pub fn new_insert_operator(table_name: String, columns: Vec<ColumnRef>) -> LogicalOperator {
        LogicalOperator::Insert(LogicalInsertOperator::new(table_name, columns))
    }
    pub fn new_values_operator(
        columns: Vec<ColumnRef>,
        tuples: Vec<Vec<ScalarValue>>,
    ) -> LogicalOperator {
        LogicalOperator::Values(LogicalValuesOperator::new(columns, tuples))
    }
    pub fn new_scan_operator(table_oid: TableOid, columns: Vec<ColumnRef>) -> LogicalOperator {
        LogicalOperator::Scan(LogicalScanOperator::new(table_oid, columns))
    }
    pub fn new_project_operator(expressions: Vec<Expr>) -> LogicalOperator {
        LogicalOperator::Project(LogicalProjectOperator::new(expressions))
    }
    pub fn new_filter_operator(predicate: Expr) -> LogicalOperator {
        LogicalOperator::Filter(LogicalFilterOperator::new(predicate))
    }
    pub fn new_limit_operator(limit: Option<usize>, offset: Option<usize>) -> LogicalOperator {
        LogicalOperator::Limit(limit::LogicalLimitOperator::new(limit, offset))
    }
    pub fn new_join_operator(join_type: JoinType, condition: Option<Expr>) -> LogicalOperator {
        LogicalOperator::Join(LogicalJoinOperator::new(join_type, condition))
    }
    pub fn new_sort_operator(order_bys: Vec<BoundOrderBy>) -> LogicalOperator {
        LogicalOperator::Sort(LogicalSortOperator::new(order_bys))
    }
}
