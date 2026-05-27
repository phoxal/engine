# phoxal-runtime-perception

Optional, profile-driven object-detection runtime.

The binary is a `Runtime` shell with `RUNTIME_ID = "perception"`. It
subscribes to the perception-role camera/depth default profiles plus
`runtime/localize/state`, `runtime/frame/tree`, and `runtime/map/revision`, then
publishes:

- `runtime/perception/detections`
- `runtime/perception/state`

The detector head is deliberately pluggable. The current `PlaceholderDetector`
is deterministic and lightweight so the cadence, revision/frame checks,
tracking, and typed contracts build and test without pulling in an ML model. A
real YOLO-class backend belongs behind the same `DetectorHead` trait.

This runtime never authorizes motion, selects goals, emits mission candidates,
or sends mission commands.
