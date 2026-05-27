use phoxal_bus::zenoh_typed::TypedSchema;
use serde::{Deserialize, Serialize};

pub const COMMAND_TOPIC: &str = "runtime/power/command";
pub const STATE_TOPIC: &str = "runtime/power/state";
pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 20,
        cpu_sustained_pct: 1,
        gpu_memory_mb: None,
    };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Poweroff,
    Reboot,
}

impl TypedSchema for Command {
    const SCHEMA_NAME: &'static str = "runtime/power/command";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub requested: Option<Command>,
    pub status: Status,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Idle,
    Accepted,
    Rejected(RejectedReason),
    Failed(FailedReason),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectedReason {
    SupervisorUnavailable,
    SupervisorReturnedHttp { code: u16 },
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailedReason {
    SupervisorTransport,
}

impl TypedSchema for State {
    const SCHEMA_NAME: &'static str = "runtime/power/state";
    const SCHEMA_VERSION: u32 = 1;
}

pub mod command {
    use super::Command;
    use phoxal_bus::pubsub::Stamped;
    use phoxal_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

    pub const TOPIC: &str = super::COMMAND_TOPIC;

    pub fn topic(bus: &phoxal_bus::Bus) -> String {
        bus.topic(TOPIC)
    }

    pub fn publisher(
        bus: &phoxal_bus::Bus,
    ) -> phoxal_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Command>>> {
        phoxal_bus::pubsub::publisher_builder(bus, TOPIC)
    }

    pub fn subscriber_builder(
        bus: &phoxal_bus::Bus,
    ) -> TypedSubscriberBuilder<'_, 'static, Stamped<Command>> {
        phoxal_bus::pubsub::subscriber_builder(bus, TOPIC)
    }
}

pub mod state {
    use super::State;
    use phoxal_bus::pubsub::Stamped;
    use phoxal_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

    pub const TOPIC: &str = super::STATE_TOPIC;

    pub fn topic(bus: &phoxal_bus::Bus) -> String {
        bus.topic(TOPIC)
    }

    pub fn publisher(
        bus: &phoxal_bus::Bus,
    ) -> phoxal_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<State>>> {
        phoxal_bus::pubsub::publisher_builder(bus, TOPIC)
    }

    pub fn subscriber_builder(
        bus: &phoxal_bus::Bus,
    ) -> TypedSubscriberBuilder<'_, 'static, Stamped<State>> {
        phoxal_bus::pubsub::subscriber_builder(bus, TOPIC)
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, State};
    use phoxal_bus::zenoh_typed::TypedSchema;

    #[test]
    fn command_contract_schema_is_stable() {
        assert_eq!(Command::SCHEMA_NAME, "runtime/power/command");
        assert_eq!(Command::SCHEMA_VERSION, 1);
    }

    #[test]
    fn state_contract_schema_is_stable() {
        assert_eq!(State::SCHEMA_NAME, "runtime/power/state");
        assert_eq!(State::SCHEMA_VERSION, 1);
    }
}
