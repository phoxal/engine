use derive_new::new;
use phoxal_bus::pubsub::Stamped;
use phoxal_bus::zenoh_typed::TypedSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct Frame {
    #[new(into)]
    data: Vec<u8>,
}

impl Frame {
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl TypedSchema for Frame {
    const SCHEMA_NAME: &'static str = "component/capability/microphone";
    const SCHEMA_VERSION: u32 = 1;
}

pub const KIND: &str = "microphone";

pub fn topic(
    bus: &phoxal_bus::Bus,
    component_id: impl AsRef<str>,
    capability_id: impl AsRef<str>,
) -> String {
    super::default_profile_topic(bus, component_id, capability_id)
}

pub fn subscriber_builder(
    bus: &phoxal_bus::Bus,
    component_id: impl AsRef<str>,
    capability_id: impl AsRef<str>,
) -> phoxal_bus::zenoh_typed::TypedSubscriberBuilder<'_, '_, Stamped<Frame>> {
    phoxal_bus::pubsub::subscriber_builder(
        bus,
        &super::default_profile_path(component_id, capability_id),
    )
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use super::Frame;

    #[test]
    fn schema_contract_does_not_drift() {
        assert_eq!(Frame::SCHEMA_NAME, "component/capability/microphone");
        assert_eq!(Frame::SCHEMA_VERSION, 1);
    }
}
