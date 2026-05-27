pub const SCHEMA_NAME: &str = "phoxal-runtime-presence-api/v1";
pub const SCHEMA_VERSION: u32 = 1;

pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 40,
        cpu_sustained_pct: 2,
        gpu_memory_mb: None,
    };

#[cfg(test)]
mod v1_version_tests {
    use super::{SCHEMA_NAME, SCHEMA_VERSION};

    #[test]
    fn api_contract_version_is_stable() {
        assert_eq!(SCHEMA_NAME, "phoxal-runtime-presence-api/v1");
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
