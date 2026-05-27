pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 100,
        cpu_sustained_pct: 5,
        gpu_memory_mb: None,
    };

#[cfg(test)]
mod tests {
    #[test]
    fn resource_budget_is_declared() {
        assert_eq!(crate::RESOURCE_BUDGET.ram_mb, 100);
        assert_eq!(crate::RESOURCE_BUDGET.cpu_sustained_pct, 5);
        assert_eq!(crate::RESOURCE_BUDGET.gpu_memory_mb, None);
    }
}
