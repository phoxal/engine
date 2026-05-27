# phoxal-runtime-perception-api

Owner-local contracts for the optional `perception` runtime.

This crate defines the typed `runtime/perception/detections` and
`runtime/perception/state` products plus supporting detection, revision-linkage,
health, and tracked-observation payloads. The detector's class labels are
model-defined output data, so `Detection::class_label` is intentionally a
`String`; health and stop/degraded reasons stay typed enums.

The API does not choose goals, authorize motion, or define a detector model.
Real detector backends and the deterministic placeholder runtime publish through
the same contracts.
