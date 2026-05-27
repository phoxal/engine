use phoxal_bus::zenoh_typed::TypedSchema;
use serde::{Deserialize, Serialize};

pub const TARGET_TOPIC: &str = "runtime/drive/target";
pub const STATE_TOPIC: &str = "runtime/drive/state";
pub const DEBUG_ACTUATOR_COMMANDS_TOPIC: &str = "runtime/drive/debug/actuator_commands";
pub const DEBUG_SATURATION_TOPIC: &str = "runtime/drive/debug/saturation";
pub const DEBUG_WATCHDOG_TOPIC: &str = "runtime/drive/debug/watchdog";
pub const DEBUG_KINEMATICS_TOPIC: &str = "runtime/drive/debug/kinematics";
pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 100,
        cpu_sustained_pct: 7,
        gpu_memory_mb: None,
    };

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub linear_x_mps: f64,
    pub angular_z_radps: f64,
}

impl TypedSchema for Target {
    const SCHEMA_NAME: &'static str = "runtime/drive/target";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub target: Target,
    pub limited_target: Target,
    pub actuator_authority: ActuatorAuthority,
    pub stop_reason: Option<StopReason>,
}

impl TypedSchema for State {
    const SCHEMA_NAME: &'static str = "runtime/drive/state";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActuatorAuthority {
    Active,
    Stopped,
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    CommandTimedOut,
    SafetyStop,
    EmergencyStop,
    NoTarget,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorCommands {
    pub commands: Vec<ActuatorCommand>,
}

impl TypedSchema for ActuatorCommands {
    const SCHEMA_NAME: &'static str = "runtime/drive/debug/actuator_commands";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorCommand {
    pub component_id: String,
    pub capability_id: String,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Saturation {
    pub requested: Target,
    pub limited: Target,
    pub reasons: Vec<String>,
}

impl TypedSchema for Saturation {
    const SCHEMA_NAME: &'static str = "runtime/drive/debug/saturation";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Watchdog {
    pub target_fresh: bool,
    pub reason: Option<String>,
}

impl TypedSchema for Watchdog {
    const SCHEMA_NAME: &'static str = "runtime/drive/debug/watchdog";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kinematics {
    pub profile: String,
    pub actuator_commands: Vec<ActuatorCommand>,
}

impl TypedSchema for Kinematics {
    const SCHEMA_NAME: &'static str = "runtime/drive/debug/kinematics";
    const SCHEMA_VERSION: u32 = 1;
}

phoxal_bus::pubsub_leaf!(target, TARGET_TOPIC, Target);
phoxal_bus::pubsub_leaf!(state, STATE_TOPIC, State);

pub mod debug {
    phoxal_bus::pubsub_leaf!(
        actuator_commands,
        DEBUG_ACTUATOR_COMMANDS_TOPIC,
        ActuatorCommands
    );
    phoxal_bus::pubsub_leaf!(saturation, DEBUG_SATURATION_TOPIC, Saturation);
    phoxal_bus::pubsub_leaf!(watchdog, DEBUG_WATCHDOG_TOPIC, Watchdog);
    phoxal_bus::pubsub_leaf!(kinematics, DEBUG_KINEMATICS_TOPIC, Kinematics);
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use super::{ActuatorCommands, Kinematics, Saturation, State, Target, Watchdog};

    #[test]
    fn schema_contracts_do_not_drift() {
        assert_eq!(Target::SCHEMA_NAME, "runtime/drive/target");
        assert_eq!(Target::SCHEMA_VERSION, 1);
        assert_eq!(State::SCHEMA_NAME, "runtime/drive/state");
        assert_eq!(State::SCHEMA_VERSION, 1);
        assert_eq!(
            ActuatorCommands::SCHEMA_NAME,
            "runtime/drive/debug/actuator_commands"
        );
        assert_eq!(ActuatorCommands::SCHEMA_VERSION, 1);
        assert_eq!(Saturation::SCHEMA_NAME, "runtime/drive/debug/saturation");
        assert_eq!(Saturation::SCHEMA_VERSION, 1);
        assert_eq!(Watchdog::SCHEMA_NAME, "runtime/drive/debug/watchdog");
        assert_eq!(Watchdog::SCHEMA_VERSION, 1);
        assert_eq!(Kinematics::SCHEMA_NAME, "runtime/drive/debug/kinematics");
        assert_eq!(Kinematics::SCHEMA_VERSION, 1);
    }
}
