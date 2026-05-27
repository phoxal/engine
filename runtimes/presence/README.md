# Robot Runtime Presence

`phoxal-runtime-presence` is the P0 readiness aggregator.

Target scope: robot discovery, service readiness summary, autonomy-readiness
summary, and compact operator-facing status.

Primary products: `runtime/presence/heartbeat`,
`runtime/presence/summary`, and `runtime/presence/debug/readiness`.

It subscribes to stamped runtime heartbeats on
`runtime/presence/heartbeat`, tracks the latest status per runtime, marks stale
heartbeats as degraded, and republishes a sorted compact readiness summary once
per step. `autonomy_ready` is always `false` until the later autonomy gate lands.
