use std::sync::Arc;

use crate::planner::{logical_plan::LogicalPlan, operator::LogicalOperator};

use self::physical_plan::PhysicalPlan;

pub mod operator;
pub mod physical_plan;

pub struct Optimizer {
    logical_plan: Arc<LogicalPlan>,
}
impl Optimizer {
    pub fn new(logical_plan: Arc<LogicalPlan>) -> Self {
        Self { logical_plan }
    }

    // 生成优化后的物理计划
    pub fn find_best(&self) -> PhysicalPlan {
        let physical_node = Self::build_physical_node(self.logical_plan.clone());
        // TODO 递归
        Self::build_physical_plan(physical_node, self.logical_plan.clone())
    }

    fn build_physical_plan(
        mut physical_plan: PhysicalPlan,
        logical_plan: Arc<LogicalPlan>,
    ) -> PhysicalPlan {
        for logical_child in logical_plan.children.iter() {
            let physical_child = Self::build_physical_node(logical_child.clone());
            physical_plan
                .children
                .push(Arc::new(Self::build_physical_plan(
                    physical_child,
                    logical_child.clone(),
                )));
        }
        physical_plan
    }

    fn build_physical_node(logical_node: Arc<LogicalPlan>) -> PhysicalPlan {
        match logical_node.operator {
            LogicalOperator::Dummy => PhysicalPlan::dummy(),
            LogicalOperator::Insert(ref logic_insert) => {
                PhysicalPlan::new_insert_node(&logic_insert.table_name, &logic_insert.columns)
            }
            LogicalOperator::Values(ref logical_values) => {
                PhysicalPlan::new_values_node(&logical_values.columns, &logical_values.tuples)
            }
        }
    }
}
