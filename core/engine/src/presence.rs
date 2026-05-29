use phoxal_infra_bus::zenoh_typed::TypedSchema;
use serde::{Deserialize, Serialize};

pub const HEARTBEAT_TOPIC: &str = "runtime/presence/heartbeat";
pub const SUMMARY_TOPIC: &str = "runtime/presence/summary";
pub const DEBUG_READINESS_TOPIC: &str = "runtime/presence/debug/readiness";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuntimeId(pub String);

impl RuntimeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for RuntimeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Readiness {
    NotStarted,
    Initializing,
    Ready,
    Degraded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heartbeat {
    pub runtime_id: RuntimeId,
    pub readiness: Readiness,
}

impl TypedSchema for Heartbeat {
    const SCHEMA_NAME: &'static str = "runtime/presence/heartbeat";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReadiness {
    pub runtime_id: RuntimeId,
    pub readiness: Readiness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    pub autonomy_ready: bool,
    pub runtimes: Vec<RuntimeReadiness>,
}

impl TypedSchema for Summary {
    const SCHEMA_NAME: &'static str = "runtime/presence/summary";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugReadiness {
    pub runtimes: Vec<RuntimeReadiness>,
}

impl TypedSchema for DebugReadiness {
    const SCHEMA_NAME: &'static str = "runtime/presence/debug/readiness";
    const SCHEMA_VERSION: u32 = 1;
}

pub mod heartbeat {
    use super::Heartbeat;
    use phoxal_infra_bus::pubsub::Stamped;
    use phoxal_infra_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

    pub const TOPIC: &str = super::HEARTBEAT_TOPIC;

    pub fn topic(bus: &phoxal_infra_bus::Bus) -> String {
        bus.topic(TOPIC)
    }

    pub fn publisher(
        bus: &phoxal_infra_bus::Bus,
    ) -> phoxal_infra_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Heartbeat>>> {
        phoxal_infra_bus::pubsub::publisher_builder(bus, TOPIC)
    }

    pub fn subscriber_builder(
        bus: &phoxal_infra_bus::Bus,
    ) -> TypedSubscriberBuilder<'_, 'static, Stamped<Heartbeat>> {
        phoxal_infra_bus::pubsub::subscriber_builder(bus, TOPIC)
    }
}

pub mod summary {
    use super::Summary;
    use phoxal_infra_bus::pubsub::Stamped;
    use phoxal_infra_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

    pub const TOPIC: &str = super::SUMMARY_TOPIC;

    pub fn topic(bus: &phoxal_infra_bus::Bus) -> String {
        bus.topic(TOPIC)
    }

    pub fn publisher(
        bus: &phoxal_infra_bus::Bus,
    ) -> phoxal_infra_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Summary>>> {
        phoxal_infra_bus::pubsub::publisher_builder(bus, TOPIC)
    }

    pub fn subscriber_builder(
        bus: &phoxal_infra_bus::Bus,
    ) -> TypedSubscriberBuilder<'_, 'static, Stamped<Summary>> {
        phoxal_infra_bus::pubsub::subscriber_builder(bus, TOPIC)
    }
}

pub mod debug {
    pub mod readiness {
        use crate::presence::DebugReadiness;
        use phoxal_infra_bus::pubsub::Stamped;
        use phoxal_infra_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

        pub const TOPIC: &str = crate::presence::DEBUG_READINESS_TOPIC;

        pub fn topic(bus: &phoxal_infra_bus::Bus) -> String {
            bus.topic(TOPIC)
        }

        pub fn publisher(
            bus: &phoxal_infra_bus::Bus,
        ) -> phoxal_infra_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<DebugReadiness>>>
        {
            phoxal_infra_bus::pubsub::publisher_builder(bus, TOPIC)
        }

        pub fn subscriber_builder(
            bus: &phoxal_infra_bus::Bus,
        ) -> TypedSubscriberBuilder<'_, 'static, Stamped<DebugReadiness>> {
            phoxal_infra_bus::pubsub::subscriber_builder(bus, TOPIC)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DebugReadiness, Heartbeat, Summary};
    use phoxal_infra_bus::zenoh_typed::TypedSchema;

    #[test]
    fn schema_contracts_do_not_drift() {
        assert_eq!(Heartbeat::SCHEMA_NAME, "runtime/presence/heartbeat");
        assert_eq!(Heartbeat::SCHEMA_VERSION, 1);
        assert_eq!(Summary::SCHEMA_NAME, "runtime/presence/summary");
        assert_eq!(Summary::SCHEMA_VERSION, 1);
        assert_eq!(
            DebugReadiness::SCHEMA_NAME,
            "runtime/presence/debug/readiness"
        );
        assert_eq!(DebugReadiness::SCHEMA_VERSION, 1);
    }
}
