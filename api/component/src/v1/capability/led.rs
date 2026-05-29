use derive_new::new;
use phoxal_infra_bus::pubsub::Stamped;
use phoxal_infra_bus::zenoh_typed::{TypedPublisherBuilder, TypedSchema};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, new)]
pub enum Command {
    On,
    Off,
}

impl TypedSchema for Command {
    const SCHEMA_NAME: &'static str = "component/capability/led";
    const SCHEMA_VERSION: u32 = 1;
}

pub const KIND: &str = "led";

pub fn topic(
    bus: &phoxal_infra_bus::Bus,
    component_id: impl AsRef<str>,
    capability_id: impl AsRef<str>,
) -> String {
    super::command_topic(bus, component_id, KIND, capability_id)
}

pub fn publisher(
    bus: &phoxal_infra_bus::Bus,
    component_id: impl AsRef<str>,
    capability_id: impl AsRef<str>,
) -> phoxal_infra_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Command>>> {
    phoxal_infra_bus::pubsub::publisher_builder(
        bus,
        &super::command_path(component_id, KIND, capability_id),
    )
}

#[cfg(test)]
mod tests {
    use phoxal_infra_bus::zenoh_typed::TypedSchema;

    use super::Command;

    #[test]
    fn schema_contract_does_not_drift() {
        assert_eq!(Command::SCHEMA_NAME, "component/capability/led");
        assert_eq!(Command::SCHEMA_VERSION, 1);
    }
}
