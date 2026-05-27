pub mod capability;
mod component;
pub mod conformance;
mod driver;
mod identity;
pub mod localize_backend;
mod motion;
pub mod profile;
pub mod resolver;
pub mod role;
pub mod role_resolution;
mod validation;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub use component::Component;
pub use driver::{ConnectionConfig, DriverConfig, GpioDirection, GpioPinConfig};
pub use identity::Identity;
pub use localize_backend::{
    LocalizeBackendKind, ResolvedLocalizeBackend, resolve_localize_backend,
};
pub use motion::{CalibrationConfig, KinematicConfig, KinematicKind, Motion, MotionLimits};
use phoxal_utils_component::v1::CapabilityRef;
pub use profile::{AutonomyProfileId, AutonomyProfileSpec, autonomy_profile};
pub use resolver::{ResolvedCapabilityRole, ResolvedFacts, SourceBundle, resolve_source_bundle};
pub use role::Role;
pub use role_resolution::{RoleAssignment, RoleResolution};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelV1 {
    pub identity: Identity,
    pub motion: Motion,
    #[serde(default)]
    pub components: BTreeMap<String, Component>,
}

impl ModelV1 {
    #[must_use]
    pub fn model(&self) -> &str {
        &self.identity.model
    }

    #[must_use]
    pub fn components(&self) -> &BTreeMap<String, Component> {
        &self.components
    }

    #[must_use]
    pub fn component_instance(&self, component_id: &str) -> Option<&Component> {
        self.components.get(component_id)
    }

    #[must_use]
    pub fn parameter(&self, capability_ref: &CapabilityRef) -> Option<&capability::Parameters> {
        self.component_instance(&capability_ref.component_id)
            .and_then(|component| component.parameters.get(&capability_ref.capability_id))
    }

    #[must_use]
    pub fn used_component_types(&self) -> BTreeSet<&str> {
        self.components
            .values()
            .map(|component| component.component.as_str())
            .collect()
    }
}
