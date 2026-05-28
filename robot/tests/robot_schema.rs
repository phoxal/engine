use std::collections::BTreeMap;

use phoxal_component::v1::CapabilityRef;
use phoxal_robot::Robot as RobotManifest;
use phoxal_robot::v1::{
    Component, ComponentSource, Components, ConnectionConfig, DriverConfig, Identity,
    KinematicConfig, Motion, Phoxal, PhoxalRuntimes, PlatformRuntimeOverride, Robot, Sim,
    SourcePath, UserRuntime, ValidationError,
};

const PLATFORM_RUNTIMES: &[&str] = &["router", "drive", "localize"];

#[test]
fn robot_roundtrips_through_yaml() {
    let robot = sample_robot();
    let yaml = serde_yaml::to_string(&RobotManifest::V1(robot.clone()))
        .expect("robot should serialize with version dispatcher");
    let reparsed = Robot::read_from_string(&yaml).expect("serialized robot should parse");

    assert_eq!(reparsed, robot);
}

#[test]
fn parses_plan_robot_fixture() {
    let robot = Robot::read_from_string(include_str!("fixtures/plan_robot.yaml"))
        .expect("plan robot fixture should parse");

    assert_eq!(robot.identity.id, "robot-v1");
    assert_eq!(robot.components.sources.len(), 3);
    robot
        .validate_with(PLATFORM_RUNTIMES)
        .expect("plan robot fixture should validate against platform names");
}

#[test]
fn unknown_platform_override_is_validation_error() {
    let mut robot = sample_robot();
    robot.phoxal_runtimes.overrides.insert(
        "not_platform".to_string(),
        PlatformRuntimeOverride {
            image: None,
            version: Some("latest".to_string()),
        },
    );

    let errors = robot
        .validate_with(PLATFORM_RUNTIMES)
        .expect_err("unknown override should fail validation");

    assert!(
        errors.contains(&ValidationError::UnknownPlatformRuntimeOverride {
            name: "not_platform".to_string()
        })
    );
}

#[test]
fn user_runtime_cannot_shadow_platform_runtime() {
    let mut robot = sample_robot();
    robot.user_runtimes.insert(
        "drive".to_string(),
        UserRuntime {
            path: "./runtimes/drive".into(),
        },
    );

    let errors = robot
        .validate_with(PLATFORM_RUNTIMES)
        .expect_err("shadowing platform runtime should fail validation");

    assert!(
        errors.contains(&ValidationError::UserRuntimeShadowsPlatformRuntime {
            name: "drive".to_string()
        })
    );
}

#[test]
fn component_instance_requires_declared_source() {
    let mut robot = sample_robot();
    robot.components.sources.remove("ddsm115");

    let errors = robot
        .validate()
        .expect_err("missing component source should fail validation");

    assert!(errors.contains(&ValidationError::MissingComponentSource {
        instance: "left_drive".to_string(),
        source: "ddsm115".to_string()
    }));
}

fn sample_robot() -> Robot {
    Robot {
        phoxal: Phoxal {
            cli_min_version: "^0.6".to_string(),
        },
        identity: Identity {
            id: "sample-bot".to_string(),
            namespace: "dev".to_string(),
        },
        structure: "structure.urdf".into(),
        phoxal_runtimes: PhoxalRuntimes {
            version: "^0.1".to_string(),
            overrides: BTreeMap::from([(
                "drive".to_string(),
                PlatformRuntimeOverride {
                    image: None,
                    version: Some("latest".to_string()),
                },
            )]),
        },
        user_runtimes: BTreeMap::from([(
            "mission_behavior".to_string(),
            UserRuntime {
                path: "./runtimes/mission_behavior".into(),
            },
        )]),
        sim: Sim {
            world: "sim/worlds/training.wbt".into(),
        },
        tools: BTreeMap::new(),
        motion: Motion {
            kinematic: KinematicConfig::Differential {
                left_actuators: vec![CapabilityRef::new("left_drive", "motor")],
                right_actuators: vec![CapabilityRef::new("right_drive", "motor")],
                left_encoders: vec![CapabilityRef::new("left_drive", "encoder")],
                right_encoders: vec![CapabilityRef::new("right_drive", "encoder")],
                wheel_radius_m: 0.12,
                wheel_base_m: 0.6,
            },
        },
        components: Components {
            sources: BTreeMap::from([(
                "ddsm115".to_string(),
                ComponentSource::Path(SourcePath {
                    path: "./components/ddsm115".into(),
                }),
            )]),
            instances: BTreeMap::from([
                (
                    "left_drive".to_string(),
                    drive_instance(1, "left_wheel_mount"),
                ),
                (
                    "right_drive".to_string(),
                    drive_instance(2, "right_wheel_mount"),
                ),
            ]),
        },
    }
}

fn drive_instance(node_id: u8, mount_link: &str) -> Component {
    Component {
        component: "ddsm115".to_string(),
        mount_link: mount_link.to_string(),
        driver: Some(DriverConfig {
            image: None,
            connection: ConnectionConfig::Can { bus: 0, node_id },
            runtime_clock_ms: 100,
        }),
        roles: BTreeMap::new(),
        parameters: BTreeMap::new(),
    }
}
