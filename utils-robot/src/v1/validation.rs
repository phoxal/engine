use std::collections::BTreeSet;

use anyhow::{Result, bail};
use phoxal_utils_component::v1::CapabilityRef;

use super::{KinematicConfig, ModelV1, capability};

impl ModelV1 {
    pub fn validate(&self) -> Result<()> {
        let mut validation_errors = Vec::new();

        self.validate_basics(&mut validation_errors);
        self.validate_component_structure(&mut validation_errors);
        self.validate_driver_structure(&mut validation_errors);
        self.validate_role_hints(&mut validation_errors);
        self.validate_kinematics(&mut validation_errors);
        self.validate_numerics(&mut validation_errors);

        if validation_errors.is_empty() {
            Ok(())
        } else {
            bail!("Model errors:\n{}", validation_errors.join("\n"))
        }
    }

    fn validate_basics(&self, validation_errors: &mut Vec<String>) {
        let model = self.identity.model.trim();
        if model.is_empty() {
            validation_errors.push("model.identity.model must not be empty".to_string());
        }
    }

    fn validate_component_structure(&self, validation_errors: &mut Vec<String>) {
        for (component_id, component) in &self.components {
            if !phoxal_utils_component::v1::is_valid_token(component_id) {
                validation_errors.push(format!(
                    "component id '{}' must contain only lowercase ASCII letters, digits, '_' or '-'",
                    component_id
                ));
            }
            if component.component.trim().is_empty() {
                validation_errors.push(format!(
                    "component '{}' must provide a non-empty component type",
                    component_id
                ));
            }
            if !phoxal_utils_component::v1::is_valid_token(&component.component) {
                validation_errors.push(format!(
                    "component '{}' has invalid component type '{}'; it must contain only lowercase ASCII letters, digits, '_' or '-'",
                    component_id, component.component
                ));
            }

            if component.mount_link.trim().is_empty() {
                validation_errors.push(format!(
                    "component '{}' has an empty mount_link",
                    component_id
                ));
            }

            for (capability_key, parameters) in &component.parameters {
                if !phoxal_utils_component::v1::is_valid_token(capability_key) {
                    validation_errors.push(format!(
                        "component '{}' parameter id '{}' must contain only lowercase ASCII letters, digits, '_' or '-'",
                        component_id, capability_key
                    ));
                }
                if parameters.kind_name().trim().is_empty() {
                    validation_errors.push(format!(
                        "components.{component_id}.parameters.{capability_key}.kind must not be empty"
                    ));
                }
            }

            for capability_id in component.roles.keys() {
                if !phoxal_utils_component::v1::is_valid_token(capability_id) {
                    validation_errors.push(format!(
                        "components.{component_id}.roles.{capability_id} must use a valid capability token"
                    ));
                }
            }
        }
    }

    fn validate_driver_structure(&self, validation_errors: &mut Vec<String>) {
        for (component_id, component) in &self.components {
            if let Some(driver) = &component.driver
                && driver.runtime_clock_ms == 0
            {
                validation_errors.push(format!(
                    "components.{component_id}.driver.runtime_clock_ms must be > 0"
                ));
            }
        }
    }

    fn validate_role_hints(&self, validation_errors: &mut Vec<String>) {
        for (component_id, component) in &self.components {
            for (capability_id, roles) in &component.roles {
                if roles.is_empty() {
                    validation_errors.push(format!(
                        "components.{component_id}.roles.{capability_id} must list at least one role"
                    ));
                }
                let mut seen = BTreeSet::new();
                for role in roles {
                    if !seen.insert(*role) {
                        validation_errors.push(format!(
                            "components.{component_id}.roles.{capability_id} repeats role '{role}'"
                        ));
                    }
                }
            }
        }
    }

    fn validate_kinematics(&self, validation_errors: &mut Vec<String>) {
        match &self.motion.kinematic {
            KinematicConfig::Differential {
                left_actuators,
                right_actuators,
                left_encoders,
                right_encoders,
                wheel_radius_m,
                wheel_base_m,
            } => {
                validate_capability_ref_list(
                    left_actuators,
                    "left_actuators",
                    "actuator",
                    validation_errors,
                );
                validate_capability_ref_list(
                    right_actuators,
                    "right_actuators",
                    "actuator",
                    validation_errors,
                );
                validate_capability_ref_list(
                    left_encoders,
                    "left_encoders",
                    "encoder",
                    validation_errors,
                );
                validate_capability_ref_list(
                    right_encoders,
                    "right_encoders",
                    "encoder",
                    validation_errors,
                );
                if !is_valid_positive_f64(*wheel_radius_m) {
                    validation_errors
                        .push("motion.kinematic.wheel_radius_m must be > 0".to_string());
                }
                if !is_valid_positive_f64(*wheel_base_m) {
                    validation_errors.push("motion.kinematic.wheel_base_m must be > 0".to_string());
                }
            }
            KinematicConfig::Mecanum {
                front_left_actuator,
                front_right_actuator,
                rear_left_actuator,
                rear_right_actuator,
                wheel_radius_m,
                wheel_base_m,
                track_m,
            } => {
                validate_capability_ref(
                    front_left_actuator,
                    "front_left_actuator",
                    validation_errors,
                );
                validate_capability_ref(
                    front_right_actuator,
                    "front_right_actuator",
                    validation_errors,
                );
                validate_capability_ref(
                    rear_left_actuator,
                    "rear_left_actuator",
                    validation_errors,
                );
                validate_capability_ref(
                    rear_right_actuator,
                    "rear_right_actuator",
                    validation_errors,
                );
                if !is_valid_positive_f64(*wheel_radius_m) {
                    validation_errors
                        .push("motion.kinematic.wheel_radius_m must be > 0".to_string());
                }
                if !is_valid_positive_f64(*wheel_base_m) {
                    validation_errors.push("motion.kinematic.wheel_base_m must be > 0".to_string());
                }
                if !is_valid_positive_f64(*track_m) {
                    validation_errors.push("motion.kinematic.track_m must be > 0".to_string());
                }
            }
            KinematicConfig::Ackermann {
                steering_actuator,
                drive_actuator,
                steering_encoder,
                drive_encoder,
                wheel_base_m,
                track_m,
                max_steering_angle_rad,
            } => {
                validate_capability_ref(steering_actuator, "steering_actuator", validation_errors);
                validate_capability_ref(drive_actuator, "drive_actuator", validation_errors);
                if let Some(capability_ref) = steering_encoder {
                    validate_capability_ref(capability_ref, "steering_encoder", validation_errors);
                }
                if let Some(capability_ref) = drive_encoder {
                    validate_capability_ref(capability_ref, "drive_encoder", validation_errors);
                }
                if !is_valid_positive_f64(*wheel_base_m) {
                    validation_errors.push("motion.kinematic.wheel_base_m must be > 0".to_string());
                }
                if !is_valid_positive_f64(*track_m) {
                    validation_errors.push("motion.kinematic.track_m must be > 0".to_string());
                }
                if !is_valid_positive_f64(*max_steering_angle_rad) {
                    validation_errors
                        .push("motion.kinematic.max_steering_angle_rad must be > 0".to_string());
                }
            }
            KinematicConfig::Omnidirectional {
                actuators,
                encoders,
            } => {
                if actuators.is_empty() {
                    validation_errors
                        .push("motion.kinematic.actuators must not be empty".to_string());
                }
                for actuator in actuators {
                    validate_capability_ref(actuator, "actuator", validation_errors);
                }
                for encoder in encoders {
                    validate_capability_ref(encoder, "encoder", validation_errors);
                }
            }
        }
    }

    fn validate_numerics(&self, validation_errors: &mut Vec<String>) {
        if !is_valid_positive_f64(self.motion.limits.max_linear_speed_mps) {
            validation_errors.push("motion.limits.max_linear_speed_mps must be > 0".to_string());
        }
        if !is_valid_positive_f64(self.motion.limits.max_angular_speed_radps) {
            validation_errors.push("motion.limits.max_angular_speed_radps must be > 0".to_string());
        }
        if !is_valid_positive_f64(self.motion.limits.max_linear_accel_mps2) {
            validation_errors.push("motion.limits.max_linear_accel_mps2 must be > 0".to_string());
        }
        if !is_valid_positive_f64(self.motion.limits.max_linear_decel_mps2) {
            validation_errors.push("motion.limits.max_linear_decel_mps2 must be > 0".to_string());
        }
        if !is_valid_positive_f64(self.motion.limits.max_angular_accel_radps2) {
            validation_errors
                .push("motion.limits.max_angular_accel_radps2 must be > 0".to_string());
        }
        for (component_id, component) in &self.components {
            for (capability_id, parameters) in &component.parameters {
                match parameters {
                    capability::Parameters::Motor(motor)
                        if motor.direction_sign != -1 && motor.direction_sign != 1 =>
                    {
                        validation_errors.push(format!(
                            "components.{component_id}.parameters.{capability_id}.direction_sign must be either -1 or 1"
                        ));
                    }
                    capability::Parameters::Encoder(sensor)
                        if sensor.direction_sign != -1 && sensor.direction_sign != 1 =>
                    {
                        validation_errors.push(format!(
                            "components.{component_id}.parameters.{capability_id}.direction_sign must be either -1 or 1"
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn validate_capability_ref(
    capability_ref: &CapabilityRef,
    field: &str,
    validation_errors: &mut Vec<String>,
) {
    if !phoxal_utils_component::v1::is_valid_token(&capability_ref.component_id)
        || !phoxal_utils_component::v1::is_valid_token(&capability_ref.capability_id)
    {
        validation_errors.push(format!(
            "motion.kinematic.{field} '{}' must use valid capability tokens",
            capability_ref
        ));
    }
}

fn validate_capability_ref_list(
    capability_refs: &[CapabilityRef],
    field: &str,
    capability_kind: &str,
    validation_errors: &mut Vec<String>,
) {
    if capability_refs.is_empty() {
        validation_errors.push(format!(
            "motion.kinematic.{field} must list at least one {capability_kind}"
        ));
    }
    for (index, capability_ref) in capability_refs.iter().enumerate() {
        validate_capability_ref(
            capability_ref,
            &format!("{field}[{index}]"),
            validation_errors,
        );
    }
}

fn is_valid_positive_f64(value: f64) -> bool {
    value.is_finite() && value > f64::EPSILON
}
