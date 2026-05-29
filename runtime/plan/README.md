# phoxal-runtime-plan

`phoxal-runtime-plan` produces the active path from the current mission goal and
the current map/localization revision pair.

The current MVP consumes `runtime/mission/goal`, localization state, and map
revision. It rejects non-planar goals and mixed map/localization revisions.

It publishes `runtime/plan/path` and `runtime/plan/state`. The path is a direct
interpolated line from the robot's current planar pose to the goal, linked to
the map revision and the localization revision the map was built from.
`runtime/plan/state` carries a typed `PlanReason` for planning/refusal causes.

Deferred: traversability cost search, obstacle avoidance, map tile queries,
map-to-odom transforms, goal selection, mission decisions, and motion execution.
