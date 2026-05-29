pub const SCHEMA_NAME: &str = "phoxal-api-presence/v1";
pub const SCHEMA_VERSION: u32 = 1;

#[cfg(test)]
mod v1_version_tests {
    use super::{SCHEMA_NAME, SCHEMA_VERSION};

    #[test]
    fn api_contract_version_is_stable() {
        assert_eq!(SCHEMA_NAME, "phoxal-api-presence/v1");
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
