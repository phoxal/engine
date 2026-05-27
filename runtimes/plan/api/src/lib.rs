use phoxal_bus::zenoh_typed::TypedSchema;
use phoxal_runtime_localize_api::LocalizationRevisionId;
use phoxal_runtime_map_api::MapRevisionId;
use phoxal_runtime_mission_api::Goal;
use serde::{Deserialize, Serialize};

pub const PATH_TOPIC: &str = "runtime/plan/path";
pub const STATE_TOPIC: &str = "runtime/plan/state";
pub const GOAL_INPUT_TOPIC: &str = phoxal_runtime_mission_api::GOAL_TOPIC;
pub const DEBUG_SEARCH_GRAPH_TOPIC: &str = "runtime/plan/debug/search_graph";
pub const DEBUG_COST_LAYERS_TOPIC: &str = "runtime/plan/debug/cost_layers";
pub const DEBUG_REJECTED_PATHS_TOPIC: &str = "runtime/plan/debug/rejected_paths";
pub const DEBUG_REVISION_INPUTS_TOPIC: &str = "runtime/plan/debug/revision_inputs";
pub const RESOURCE_BUDGET: phoxal_utils_conventions::resource::RuntimeBudget =
    phoxal_utils_conventions::resource::RuntimeBudget {
        ram_mb: 125,
        cpu_sustained_pct: 10,
        gpu_memory_mb: None,
    };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Path {
    pub goal: Goal,
    pub map_revision: MapRevisionId,
    pub built_from_localize_revision: LocalizationRevisionId,
    pub frame_id: String,
    pub poses: Vec<PathPose>,
}

impl TypedSchema for Path {
    const SCHEMA_NAME: &'static str = "runtime/plan/path";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathPose {
    pub xy_m: [f64; 2],
    pub yaw_rad: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub status: PlanStatus,
    pub reason: Option<PlanReason>,
}

impl TypedSchema for State {
    const SCHEMA_NAME: &'static str = "runtime/plan/state";
    const SCHEMA_VERSION: u32 = 2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Idle,
    Planning,
    Ready,
    Failed,
    Refused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanReason {
    NonPlanarGoalUnsupported,
    NoLocalizationState,
    LocalizationInitializing,
    LocalizationLost,
    LocalizationRelocalizing,
    UnsupportedLocalizationMode,
    NoLocalizationPose,
    NoLocalizationRevision,
    NoMapRevision,
    GoalMapRevisionMismatch,
    MapLocalizeRevisionMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<[String; 2]>,
}

impl TypedSchema for SearchGraph {
    const SCHEMA_NAME: &'static str = "runtime/plan/debug/search_graph";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostLayers {
    pub layers: Vec<CostLayer>,
}

impl TypedSchema for CostLayers {
    const SCHEMA_NAME: &'static str = "runtime/plan/debug/cost_layers";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostLayer {
    pub name: String,
    pub weight: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedPaths {
    pub rejected: Vec<RejectedPath>,
}

impl TypedSchema for RejectedPaths {
    const SCHEMA_NAME: &'static str = "runtime/plan/debug/rejected_paths";
    const SCHEMA_VERSION: u32 = 1;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedPath {
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionInputs {
    pub map_revision: Option<MapRevisionId>,
    pub localization_revision: Option<LocalizationRevisionId>,
}

impl TypedSchema for RevisionInputs {
    const SCHEMA_NAME: &'static str = "runtime/plan/debug/revision_inputs";
    const SCHEMA_VERSION: u32 = 1;
}

phoxal_bus::pubsub_leaf!(path, PATH_TOPIC, Path);
phoxal_bus::pubsub_leaf!(state, STATE_TOPIC, State);

pub mod debug {
    phoxal_bus::pubsub_leaf!(search_graph, DEBUG_SEARCH_GRAPH_TOPIC, SearchGraph);
    phoxal_bus::pubsub_leaf!(cost_layers, DEBUG_COST_LAYERS_TOPIC, CostLayers);
    phoxal_bus::pubsub_leaf!(rejected_paths, DEBUG_REJECTED_PATHS_TOPIC, RejectedPaths);
    phoxal_bus::pubsub_leaf!(revision_inputs, DEBUG_REVISION_INPUTS_TOPIC, RevisionInputs);
}

#[cfg(test)]
mod tests {
    use phoxal_bus::zenoh_typed::TypedSchema;

    use super::{CostLayers, Path, RejectedPaths, RevisionInputs, SearchGraph, State};

    #[test]
    fn schema_contracts_do_not_drift() {
        assert_eq!(Path::SCHEMA_NAME, "runtime/plan/path");
        assert_eq!(Path::SCHEMA_VERSION, 1);
        assert_eq!(State::SCHEMA_NAME, "runtime/plan/state");
        assert_eq!(State::SCHEMA_VERSION, 2);
        assert_eq!(SearchGraph::SCHEMA_NAME, "runtime/plan/debug/search_graph");
        assert_eq!(SearchGraph::SCHEMA_VERSION, 1);
        assert_eq!(CostLayers::SCHEMA_NAME, "runtime/plan/debug/cost_layers");
        assert_eq!(CostLayers::SCHEMA_VERSION, 1);
        assert_eq!(
            RejectedPaths::SCHEMA_NAME,
            "runtime/plan/debug/rejected_paths"
        );
        assert_eq!(RejectedPaths::SCHEMA_VERSION, 1);
        assert_eq!(
            RevisionInputs::SCHEMA_NAME,
            "runtime/plan/debug/revision_inputs"
        );
        assert_eq!(RevisionInputs::SCHEMA_VERSION, 1);
    }
}
