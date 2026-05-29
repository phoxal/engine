//! `phoxal-runtime-localize` library surface: exposes the backend selector and
//! runtime backend types so scenario tests can exercise the real selector
//! logic without spawning the binary.
//!
//! The binary entrypoint is `src/main.rs`; this lib target shares the same
//! source modules.

pub mod conformance;
mod geodetic;
mod gnss_anchored;
pub mod orbslam3;
mod pose_math;
mod registration;
pub mod runtime;
pub mod scenarios;
pub mod selector;
mod settings;
mod sim_truth;
