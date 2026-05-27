# Robot Model

Versioned source-model data types for authored robot facts.

## Overview

`phoxal-utils-robot` owns the v1 source schema and the shared deterministic
foundation contracts:

- `Model` is the versioned enum stored in `model.yaml`.
- `ModelV1` describes robot facts: identity, motion, component instances,
  driver connections, capability parameters, and model-instance role hints.
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
`connection`. Motion limits and kinematics remain durable robot facts. Timing,
safety margins, runtime presence, image selection, and deploy topology are not
source-model runtime config.

## Usage

```rust
use phoxal_utils_robot::Model;

let model: Model = serde_yaml::from_str(include_str!("model.yaml"))?;
let model = model.as_v1().expect("expected v1 schema");
```
