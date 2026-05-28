//! Simulator robot-pose wire contract.

use phoxal_bus::pubsub::Stamped;
use phoxal_bus::zenoh_typed::{TypedPublisherBuilder, TypedSchema, TypedSubscriberBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pose {
    pub frame_id: String,
    pub translation_m: [f64; 3],
    pub rotation_xyzw: [f64; 4],
}

impl TypedSchema for Pose {
    const SCHEMA_NAME: &'static str = "simulation/robot/pose";
    const SCHEMA_VERSION: u32 = 1;
}

pub const SCHEMA: &str = "simulation/robot/pose";

pub fn path(robot_id: impl AsRef<str>) -> String {
    format!("simulation/robot/{}/pose", robot_id.as_ref())
}

pub fn topic(bus: &phoxal_bus::Bus, robot_id: impl AsRef<str>) -> String {
    bus.topic(&path(robot_id))
}

pub fn publisher(
    bus: &phoxal_bus::Bus,
    robot_id: impl AsRef<str>,
) -> phoxal_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Pose>>> {
    phoxal_bus::pubsub::publisher_builder(bus, &path(robot_id))
}

pub fn subscriber_builder(
    bus: &phoxal_bus::Bus,
    robot_id: impl AsRef<str>,
) -> TypedSubscriberBuilder<'_, 'static, Stamped<Pose>> {
    phoxal_bus::pubsub::subscriber_builder(bus, &path(robot_id))
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use super::{Pose, SCHEMA};

    #[test]
    fn pose_contract_matches_simulator_wire_values() {
        assert_eq!(Pose::SCHEMA_NAME, "simulation/robot/pose");
        assert_eq!(Pose::SCHEMA_VERSION, 1);
        assert_eq!(SCHEMA, "simulation/robot/pose");
        assert_eq!(super::path("robot-v1"), "simulation/robot/robot-v1/pose");
    }
}
