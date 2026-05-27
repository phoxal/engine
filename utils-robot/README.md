# Robot Manifest

Source robot manifest data types for authored robot facts.

## Overview

`phoxal-utils-robot` owns the `robot.yaml` source schema and the shared
deterministic foundation contracts:

- `Robot` is the single top-level type stored in `robot.yaml`.
- `Robot` describes identity, runtime resolution intent, simulation intent,
  motion kinematics, component sources, mounted component instances, driver
  connections, capability parameters, and model-instance role hints.
- Source models do not author the runtime graph or per-runtime wiring.
- Component `roles` map capability ids to role hints such as `localization`,
  `mapping`, `traversability`, `safety`, `odometry`, and optional `perception`.
- `unseen_ground_navigation` is the base first-release autonomy profile;
  `unseen_ground_navigation_night` inherits and tightens it (composition by
  extension — a child adds/tightens requirements, never removes a parent's).
- Role resolution, profile conformance, shared resolved facts, and deploy
  descriptor types live here; runtime-specific resolved slices belong to the
  owning runtime crates in later phases.

Component-driver config remains per component instance and includes an explicit
`connection`. Timing, safety margins, runtime presence, and deploy topology are
not source-manifest runtime config.

## Usage

```rust
use phoxal_utils_robot::Robot;

let robot: Robot = serde_yaml::from_str(include_str!("robot.yaml"))?;
robot.validate_with(&["router", "drive", "localize"])?;
```
