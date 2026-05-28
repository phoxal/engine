//! Simulator status wire contract.

use phoxal_bus::pubsub::Stamped;
use phoxal_bus::zenoh_typed::{TypedPublisherBuilder, TypedSchema, TypedSubscriberBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Status {
    pub epoch: u64,
    pub step: u64,
    pub time_ns: u64,
    pub dt_ns: u64,
}

impl TypedSchema for Status {
    const SCHEMA_NAME: &'static str = "simulation/status";
    const SCHEMA_VERSION: u32 = 1;
}

pub const TOPIC: &str = "simulation/status";

pub fn topic(bus: &phoxal_bus::Bus) -> String {
    bus.topic(TOPIC)
}

pub fn publisher(
    bus: &phoxal_bus::Bus,
) -> phoxal_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Status>>> {
    phoxal_bus::pubsub::publisher_builder(bus, TOPIC)
}

pub fn subscriber_builder(
    bus: &phoxal_bus::Bus,
) -> TypedSubscriberBuilder<'_, 'static, Stamped<Status>> {
    phoxal_bus::pubsub::subscriber_builder(bus, TOPIC)
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use super::{Status, TOPIC};

    #[test]
    fn status_contract_matches_simulator_wire_values() {
        assert_eq!(Status::SCHEMA_NAME, "simulation/status");
        assert_eq!(Status::SCHEMA_VERSION, 1);
        assert_eq!(TOPIC, "simulation/status");
    }
}
