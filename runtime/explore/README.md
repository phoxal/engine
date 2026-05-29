# phoxal-runtime-explore

`phoxal-runtime-explore` is a BLUEPRINT skeleton for choosing reachable
exploration objectives. It runs as a `Runtime`, consumes map-owned
traversability, map revision, and localization state, then publishes frontier
groups and scored candidate goals.

Frontiers are free traversability cells adjacent to unknown space, grouped by
4-neighbor connectivity. Goal candidates are frontier centroids in reachable
free cells, scored by frontier size and distance from the current robot pose.

Primary products: `runtime/explore/frontiers`,
`runtime/explore/goal_candidates`, and `runtime/explore/state`.

Not in scope: path execution, final goal selection, mission ownership, or map
semantics.
