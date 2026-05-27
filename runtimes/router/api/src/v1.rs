pub const SCHEMA_NAME: &str = "phoxal-runtime-router-api/v1";
pub const SCHEMA_VERSION: u32 = 1;

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
        assert_eq!(crate::v1::RESOURCE_BUDGET.ram_mb, 100);
        assert_eq!(crate::v1::RESOURCE_BUDGET.cpu_sustained_pct, 5);
        assert_eq!(crate::v1::RESOURCE_BUDGET.gpu_memory_mb, None);
    }
}

#[cfg(test)]
mod v1_version_tests {
    use super::{SCHEMA_NAME, SCHEMA_VERSION};

    #[test]
    fn api_contract_version_is_stable() {
        assert_eq!(SCHEMA_NAME, "phoxal-runtime-router-api/v1");
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
