# phoxal-runtime-map

`phoxal-runtime-map` is the retained world model, revisioned submap owner,
spatial query service, and planner-facing traversability view.

The current implementation uses the shared `Runtime` bootstrap and owns the
map revision lifecycle from `runtime/localize/revision`. Each new localization
revision records and publishes a linked `MapRevision`, keeps the current
revision plus the two previous completed revisions, and resets the map epoch
when the localization epoch changes. Until real submap building lands, retained
revision queries return `Ok` with empty payloads; evicted revisions return
`StaleRevision`, future revisions return `RevisionUnavailable`, and mismatched
epochs return `WrongEpoch`.

- Inputs: localization revision/state/keyframes, frame tree, and configured map
  sensor capability data
- Publishes: `runtime/map/revision`, `runtime/map/summary`,
  `runtime/map/local_cost`, `runtime/map/traversability`, and
  `runtime/map/traversability_summary`
- Queries: `runtime/map/query/{submap,esdf_tile,traversability_tile,local_grid,global_grid,snapshot}`
- Owns map revisions and traversability semantics over the retained world model
- Not in scope: pose estimation, mission behavior, rolling fixed-extent local
  occupancy as the target product, or Rerun-invented map semantics
