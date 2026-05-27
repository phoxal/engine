//! Scenario spec + report data types.
//!
//! Foundation crate; no runtime or simulator dependencies. Consumed by
//! orchestration tools, scenario harness crates, and crates that need to
//! read or write scenario reports.

pub mod definition;
pub mod records;

pub use definition::{ScenarioSpec, WebotsWorld};
pub use records::{ScenarioOutcome, ScenarioReport, ScenarioResult};
