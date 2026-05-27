//! Data types for authored source robot models.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod transform;
pub mod v1;

const MODEL_FILE: &str = "model.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum Model {
    #[serde(rename = "v1")]
    V1(v1::ModelV1),
}

impl Model {
    pub fn read_from_dir(path: impl AsRef<Path>) -> Result<Model> {
        let path = path.as_ref();
        Self::read_from_string(
            &std::fs::read_to_string(path.join(MODEL_FILE)).with_context(|| {
                format!(
                    "failed to read model file {}",
                    path.join(MODEL_FILE).display()
                )
            })?,
        )
    }

    pub fn read_from_string(string: &str) -> Result<Model> {
        let model = Self::parse_from_string(string)?;
        model.validate()?;
        Ok(model)
    }

    pub fn parse_from_dir(path: impl AsRef<Path>) -> Result<Model> {
        let path = path.as_ref();
        Self::parse_from_string(
            &std::fs::read_to_string(path.join(MODEL_FILE)).with_context(|| {
                format!(
                    "failed to read model file {}",
                    path.join(MODEL_FILE).display()
                )
            })?,
        )
    }

    pub fn parse_from_string(string: &str) -> Result<Model> {
        serde_yaml::from_str(string).with_context(|| "failed to parse model")
    }

    pub fn write_to_dir(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        std::fs::create_dir_all(path)
            .with_context(|| format!("failed to create model directory {}", path.display()))?;
        let yaml = serde_yaml::to_string(&self)?;
        std::fs::write(path.join(MODEL_FILE), yaml).with_context(|| {
            format!(
                "failed to write model file {}",
                path.join(MODEL_FILE).display()
            )
        })?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Model::V1(m) => m.validate(),
        }
    }
}

impl Model {
    pub fn as_v1(&self) -> Option<&v1::ModelV1> {
        match self {
            Model::V1(v1) => Some(v1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Model;
    use tempfile::tempdir;

    fn valid_model_yaml(model: &str) -> String {
        format!(
            r#"
version: v1
identity:
  model: {model}
components:
  left_drive:
    component: ddsm115
    mount_link: left_wheel_mount
    driver:
      connection:
        type: can
        bus: 0
        node_id: 1
    parameters:
      motor:
        kind: motor
        direction_sign: 1
      encoder:
        kind: encoder
        direction_sign: 1
  right_drive:
    component: ddsm115
    mount_link: right_wheel_mount
    driver:
      connection:
        type: can
        bus: 0
        node_id: 2
    parameters:
      motor:
        kind: motor
        direction_sign: -1
      encoder:
        kind: encoder
        direction_sign: -1
motion:
  kinematic:
    kind: differential
    left_actuators: [left_drive.motor]
    right_actuators: [right_drive.motor]
    left_encoders: [left_drive.encoder]
    right_encoders: [right_drive.encoder]
    wheel_radius_m: 0.1
    wheel_base_m: 0.5
  limits:
    max_linear_speed_mps: 0.8
    max_angular_speed_radps: 2.0
    max_linear_accel_mps2: 10.0
    max_linear_decel_mps2: 10.0
    max_angular_accel_radps2: 30.0

"#
        )
    }

    #[test]
    fn deserialize_versioned_model() {
        let parsed =
            Model::read_from_string(&valid_model_yaml("robot-v1")).expect("model should parse");
        assert_eq!(
            parsed
                .as_v1()
                .expect("model version should be supported")
                .identity
                .model,
            "robot-v1"
        );
    }

    #[test]
    fn model_roundtrips_through_directory() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let model_dir = temp_dir.path().join("robot");
        let model = Model::read_from_string(&valid_model_yaml("robot-v1"))?;

        model.write_to_dir(&model_dir)?;
        let loaded = Model::read_from_dir(&model_dir)?;

        assert_eq!(
            loaded
                .as_v1()
                .expect("model version should be supported")
                .identity
                .model,
            "robot-v1"
        );
        Ok(())
    }

    #[test]
    fn component_driver_requires_connection() {
        let yaml = valid_model_yaml("robot-v1").replace(
            "    driver:\n      connection:\n        type: can\n        bus: 0\n        node_id: 1\n",
            "    driver:\n      runtime_clock_ms: 20\n",
        );

        let error = Model::read_from_string(&yaml).expect_err("model should fail");
        let message = format!("{error:#}");
        assert!(message.contains("missing field `connection`"), "{message}");
    }

    #[test]
    fn model_accepts_mecanum_kinematic_config() {
        let yaml = valid_model_yaml("robot-v2").replace(
            r#"  kinematic:
    kind: differential
    left_actuators: [left_drive.motor]
    right_actuators: [right_drive.motor]
    left_encoders: [left_drive.encoder]
    right_encoders: [right_drive.encoder]
    wheel_radius_m: 0.1
    wheel_base_m: 0.5"#,
            r#"  kinematic:
    kind: mecanum
    front_left_actuator: front_left_drive.motor
    front_right_actuator: front_right_drive.motor
    rear_left_actuator: rear_left_drive.motor
    rear_right_actuator: rear_right_drive.motor
    wheel_radius_m: 0.1
    wheel_base_m: 0.5
    track_m: 0.4"#,
        );

        let parsed = Model::read_from_string(&yaml).expect("mecanum model should parse");

        assert_eq!(
            parsed
                .as_v1()
                .expect("model version should be supported")
                .motion
                .kinematic
                .kind()
                .as_str(),
            "mecanum"
        );
    }
}
