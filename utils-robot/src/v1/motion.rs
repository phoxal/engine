use serde::{Deserialize, Serialize};

use phoxal_utils_component::v1::CapabilityRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motion {
    pub kinematic: KinematicConfig,
    pub limits: MotionLimits,
    #[serde(default)]
    pub calibration: Option<CalibrationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum KinematicConfig {
    Differential {
        left_actuators: Vec<CapabilityRef>,
        right_actuators: Vec<CapabilityRef>,
        left_encoders: Vec<CapabilityRef>,
        right_encoders: Vec<CapabilityRef>,
        wheel_radius_m: f64,
        wheel_base_m: f64,
    },
    Mecanum {
        front_left_actuator: CapabilityRef,
        front_right_actuator: CapabilityRef,
        rear_left_actuator: CapabilityRef,
        rear_right_actuator: CapabilityRef,
        wheel_radius_m: f64,
        wheel_base_m: f64,
        track_m: f64,
    },
    Ackermann {
        steering_actuator: CapabilityRef,
        drive_actuator: CapabilityRef,
        steering_encoder: Option<CapabilityRef>,
        drive_encoder: Option<CapabilityRef>,
        wheel_base_m: f64,
        track_m: f64,
        max_steering_angle_rad: f64,
    },
    Omnidirectional {
        actuators: Vec<CapabilityRef>,
        encoders: Vec<CapabilityRef>,
    },
}

impl KinematicConfig {
    pub const fn kind(&self) -> KinematicKind {
        match self {
            Self::Differential { .. } => KinematicKind::Differential,
            Self::Mecanum { .. } => KinematicKind::Mecanum,
            Self::Ackermann { .. } => KinematicKind::Ackermann,
            Self::Omnidirectional { .. } => KinematicKind::Omnidirectional,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KinematicKind {
    Differential,
    Mecanum,
    Ackermann,
    Omnidirectional,
}

impl KinematicKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Differential => "differential",
            Self::Mecanum => "mecanum",
            Self::Ackermann => "ackermann",
            Self::Omnidirectional => "omnidirectional",
        }
    }
}

impl std::fmt::Display for KinematicKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotionLimits {
    pub max_linear_speed_mps: f64,
    pub max_angular_speed_radps: f64,
    pub max_linear_accel_mps2: f64,
    pub max_linear_decel_mps2: f64,
    pub max_angular_accel_radps2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationConfig {
    #[serde(default)]
    pub wheel_radius_left_m: Option<f64>,
    #[serde(default)]
    pub wheel_radius_right_m: Option<f64>,
}
