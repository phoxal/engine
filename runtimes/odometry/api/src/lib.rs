use phoxal_bus::zenoh_typed::TypedSchema;
use phoxal_runtime_frame_api::FrameId;
use phoxal_runtime_joint_api::JointId;
use serde::{Deserialize, Serialize};

pub const DATA_TOPIC: &str = "runtime/odometry/data";
pub const STATUS_TOPIC: &str = "runtime/odometry/status";
pub const DEBUG_SOURCE_HEALTH_TOPIC: &str = "runtime/odometry/debug/source_health";
pub const DEBUG_RESIDUALS_TOPIC: &str = "runtime/odometry/debug/residuals";
pub const DEBUG_INTEGRATION_TOPIC: &str = "runtime/odometry/debug/integration";
pub const RESOURCE_BUDGET: phoxal_engine::resource::RuntimeBudget =
    phoxal_engine::resource::RuntimeBudget {
        ram_mb: 80,
        cpu_sustained_pct: 7,
        gpu_memory_mb: None,
    };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OdometryEstimate {
    pub pose: PoseEstimate,
    pub velocity: VelocityEstimate,
    pub covariance: Option<Covariance>,
    pub status: Status,
}

impl TypedSchema for OdometryEstimate {
    const SCHEMA_NAME: &'static str = "runtime/odometry/data";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoseEstimate {
    pub frame_id: FrameId,
    pub child_frame_id: FrameId,
    pub translation_m: [f64; 3],
    pub rotation_xyzw: [f64; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VelocityEstimate {
    pub frame_id: FrameId,
    pub linear_mps: [f64; 3],
    pub angular_radps: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Covariance {
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    pub mode: StatusMode,
    pub reasons: Vec<StatusReason>,
}

impl TypedSchema for Status {
    const SCHEMA_NAME: &'static str = "runtime/odometry/status";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusMode {
    Initializing,
    Tracking,
    Degraded,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StatusReason {
    /// A required joint stream is missing or outside its freshness window.
    JointStale,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHealth {
    pub sources: Vec<SourceStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceStatus {
    pub source_id: SourceId,
    pub healthy: bool,
    pub reason: Option<SourceReason>,
}

impl TypedSchema for SourceHealth {
    const SCHEMA_NAME: &'static str = "runtime/odometry/debug/source_health";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SourceId {
    /// Joint position stream from the joint runtime.
    Joint(JointId),
    /// IMU stream when odometry fuses body attitude.
    Imu,
    /// Raw encoder stream when odometry consumes encoder data directly.
    Encoder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SourceReason {
    /// The source produced data before but is outside its freshness window.
    Stale,
    /// The source has not produced the sample required for tracking.
    Missing,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Residuals {
    pub residuals: Vec<Residual>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Residual {
    pub source_id: SourceId,
    pub value: f64,
}

impl TypedSchema for Residuals {
    const SCHEMA_NAME: &'static str = "runtime/odometry/debug/residuals";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Integration {
    pub steps: Vec<IntegrationStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrationStep {
    pub source_id: SourceId,
    pub delta_pose_m: [f64; 3],
    pub delta_yaw_rad: f64,
}

impl TypedSchema for Integration {
    const SCHEMA_NAME: &'static str = "runtime/odometry/debug/integration";
    const SCHEMA_VERSION: u32 = 1;
}

phoxal_bus::pubsub_leaf!(data, DATA_TOPIC, OdometryEstimate);
phoxal_bus::pubsub_leaf!(status, STATUS_TOPIC, Status);

pub mod debug {
    phoxal_bus::pubsub_leaf!(source_health, DEBUG_SOURCE_HEALTH_TOPIC, SourceHealth);
    pub mod residuals {
        use phoxal_bus::pubsub::Stamped;
        use phoxal_bus::zenoh_typed::{TypedPublisherBuilder, TypedSubscriberBuilder};

        use crate::Residuals;

        pub const TOPIC: &str = crate::DEBUG_RESIDUALS_TOPIC;

        pub fn topic(bus: &phoxal_bus::Bus) -> String {
            bus.topic(TOPIC)
        }

        /// Residuals intentionally stay empty for v1 until IMU fusion provides a reference signal.
        pub fn publisher(
            bus: &phoxal_bus::Bus,
        ) -> phoxal_bus::Result<TypedPublisherBuilder<'_, 'static, Stamped<Residuals>>> {
            phoxal_bus::pubsub::publisher_builder(bus, TOPIC)
        }

        pub fn subscriber_builder(
            bus: &phoxal_bus::Bus,
        ) -> TypedSubscriberBuilder<'_, 'static, Stamped<Residuals>> {
            phoxal_bus::pubsub::subscriber_builder(bus, TOPIC)
        }
    }
    phoxal_bus::pubsub_leaf!(integration, DEBUG_INTEGRATION_TOPIC, Integration);
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use crate::{Integration, OdometryEstimate, Residuals, SourceHealth, Status};

    #[test]
    fn odometry_estimate_schema_is_stable() {
        assert_eq!(OdometryEstimate::SCHEMA_NAME, "runtime/odometry/data");
        assert_eq!(OdometryEstimate::SCHEMA_VERSION, 1);
    }

    #[test]
    fn status_schema_is_stable() {
        assert_eq!(Status::SCHEMA_NAME, "runtime/odometry/status");
        assert_eq!(Status::SCHEMA_VERSION, 1);
    }

    #[test]
    fn source_health_schema_is_stable() {
        assert_eq!(
            SourceHealth::SCHEMA_NAME,
            "runtime/odometry/debug/source_health"
        );
        assert_eq!(SourceHealth::SCHEMA_VERSION, 1);
    }

    #[test]
    fn residuals_schema_is_stable() {
        assert_eq!(Residuals::SCHEMA_NAME, "runtime/odometry/debug/residuals");
        assert_eq!(Residuals::SCHEMA_VERSION, 1);
    }

    #[test]
    fn integration_schema_is_stable() {
        assert_eq!(
            Integration::SCHEMA_NAME,
            "runtime/odometry/debug/integration"
        );
        assert_eq!(Integration::SCHEMA_VERSION, 1);
    }
}
