# phoxal-runtime-localize

`phoxal-runtime-localize` is the BLUEPRINT navigation-state estimator and
localization revision owner.

The current implementation uses the shared `Runtime` bootstrap and ships the
v1 contract framework with dead-reckoning and GNSS-anchored backends. The
dead-reckoning backend consumes `runtime/odometry/data`. The GNSS-anchored
backend consumes the resolved GNSS capability and interprets samples according
to that capability's coordinate system: `local` passes the sample through as
local meters, while `wgs84` anchors the first geodetic fix as the ENU origin and
converts later fixes into local East/North/Up meters. The runtime publishes
`runtime/localize/state` every 20 ms and publishes `runtime/localize/pose` when
a pose is available. The ORB-SLAM3 RGB-D + inertial backend, keyframe store,
pose graph, and correction publication land in a follow-up phase.

Owned topics: `runtime/localize/{state,pose,revision,keyframe,correction}` and
pose-graph/keyframe/correction queries described in `docs/BLUEPRINT.md` and
`docs/BLUEPRINT_CONTRACTS.md`. The dead-reckoning backend has no pose graph or
corrections, so query handlers return revision-unavailable responses unless the
request is for the wrong epoch.

Scope: pose, velocity, covariance, localization mode, estimator status,
keyframes, pose-graph corrections, and revision lineage.

Not in scope: retained map ownership, mission behavior, drive arbitration, or
operator visualization semantics.
