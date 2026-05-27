use anyhow::{Result, bail};
use phoxal_utils_robot::Robot;
use phoxal_utils_robot::v1::KinematicKind;

const ALL_KINEMATICS: &[KinematicKind] = &[
    KinematicKind::Differential,
    KinematicKind::Mecanum,
    KinematicKind::Ackermann,
    KinematicKind::Omnidirectional,
];
const DIFFERENTIAL_ONLY: &[KinematicKind] = &[KinematicKind::Differential];
const MAP_DEPENDENCIES: &[RuntimeServiceKind] =
    &[RuntimeServiceKind::Localize, RuntimeServiceKind::Frame];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeServiceKind {
    Router,
    Asset,
    Presence,
    Power,
    Video,
    Joint,
    Frame,
    Odometry,
    Localize,
    Map,
    Safety,
    Mission,
    Explore,
    Plan,
    Follow,
    Motion,
    Drive,
}

impl RuntimeServiceKind {
    pub const fn config_path(self) -> &'static str {
        match self {
            Self::Router => "router",
            Self::Asset => "asset",
            Self::Presence => "presence",
            Self::Power => "power",
            Self::Video => "video",
            Self::Joint => "joint",
            Self::Frame => "frame",
            Self::Odometry => "odometry",
            Self::Localize => "localize",
            Self::Map => "map",
            Self::Safety => "safety",
            Self::Mission => "mission",
            Self::Explore => "explore",
            Self::Plan => "plan",
            Self::Follow => "follow",
            Self::Motion => "motion",
            Self::Drive => "drive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSupportMode {
    Direct,
    Independent,
    DependsOn(&'static [RuntimeServiceKind]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeSupport {
    pub service: RuntimeServiceKind,
    pub supported_kinematics: &'static [KinematicKind],
    pub mode: RuntimeSupportMode,
}

pub const RUNTIME_SUPPORT: &[RuntimeSupport] = &[
    RuntimeSupport {
        service: RuntimeServiceKind::Router,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Asset,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Presence,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Power,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Video,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Joint,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Frame,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Odometry,
        supported_kinematics: DIFFERENTIAL_ONLY,
        mode: RuntimeSupportMode::Direct,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Localize,
        supported_kinematics: DIFFERENTIAL_ONLY,
        mode: RuntimeSupportMode::Direct,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Map,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::DependsOn(MAP_DEPENDENCIES),
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Safety,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Mission,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Explore,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Plan,
        supported_kinematics: DIFFERENTIAL_ONLY,
        mode: RuntimeSupportMode::Direct,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Follow,
        supported_kinematics: DIFFERENTIAL_ONLY,
        mode: RuntimeSupportMode::Direct,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Motion,
        supported_kinematics: ALL_KINEMATICS,
        mode: RuntimeSupportMode::Independent,
    },
    RuntimeSupport {
        service: RuntimeServiceKind::Drive,
        supported_kinematics: DIFFERENTIAL_ONLY,
        mode: RuntimeSupportMode::Direct,
    },
];

pub fn validate_runtime_support(model: &Robot) -> Result<()> {
    let kinematic = model.motion.kinematic.kind();
    for support in RUNTIME_SUPPORT {
        if !support.supported_kinematics.contains(&kinematic) {
            bail!(
                "{} does not support {} kinematics; supported kinematics: {}",
                support.service.config_path(),
                kinematic,
                format_kinematic_kinds(support.supported_kinematics)
            );
        }
    }
    Ok(())
}

fn format_kinematic_kinds(kinds: &[KinematicKind]) -> String {
    kinds
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}
