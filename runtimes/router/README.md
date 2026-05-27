# phoxal-runtime-router

`phoxal-runtime-router` is the per-robot Zenoh router binary.

## Current Role

- Each robot stack runs one `phoxal-runtime-router`.
- Robot runtimes and drivers inside the stack connect only to that local router.
- The router optionally connects upstream to shared infrastructure through
  `--upstream-router ...` or `UPSTREAM_ROUTERS=tcp/name:port,tcp/name2:port`.

## Local Development

- In `cargo xtask webots up ...`, the per-robot router is exposed to host-local
  tools and runtimes at `tcp/127.0.0.1:7447`.
