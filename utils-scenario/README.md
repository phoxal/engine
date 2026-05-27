# phoxal-utils-scenario

`phoxal-utils-scenario` contains shared harness helpers for
framework-conformance and robot-acceptance scenario runners.

It owns environment-derived scenario context, passive logical-time waits,
simulator reset/status/pose reads, mission command helpers, and typed assertion
helpers. Scenario definitions and catalog ownership remain with the runner that
uses the crate.

Scenario specs and serializable scenario reports live in the foundation crate
`phoxal-utils-scenario-spec`. This crate re-exports those modules so harness
consumers can keep using `phoxal_utils_scenario::definition` and
`phoxal_utils_scenario::records`.

Runtime and robot binaries publish scenario metadata with
`ScenarioDescriptor`; Webots-backed scenarios declare their world through
`ScenarioKind::Webots`:

```rust
use std::borrow::Cow;

use phoxal_engine::step::{ScenarioDescriptor, ScenarioKind};

pub const SCENARIOS: &[ScenarioDescriptor] = &[ScenarioDescriptor {
    name: Cow::Borrowed("drive-forward"),
    summary: Cow::Borrowed("Robot moves forward to a goal pose."),
    kind: ScenarioKind::Webots {
        world: Cow::Borrowed("ArenaWorld"),
    },
    phase: phoxal_engine::step::Phase::P5,
    timeout_secs: 60,
    category: Cow::Borrowed("mission"),
    tier: 3,
}];
```

Scenario tests create `ScenarioContext::from_env()`, then use the existing
simulation and mission contracts through helper methods such as
`wait_until_ready`, `reset_simulation`, `advance_for_secs`, `simulation_pose`,
and `publish_navigate_to`.
