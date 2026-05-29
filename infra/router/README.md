# Router Infrastructure

This directory intentionally has no Dockerfile. Phoxal uses the upstream
`eclipse/zenoh` Docker image from Docker Hub directly for the on-robot router
and for the host-side local router used during simulation.

The pinned image tag and digest live outside this repository in
`phoxal-cli/src/local_zenoh.rs` as `const ZENOH_IMAGE`. Bump that constant in
`phoxal-cli` when the zenoh image needs to move; this framework repository only
documents the policy and carries the robot-side runtime Dockerfile elsewhere.

## Simulation Topology

Simulation runs two routers. The robot compose project includes an on-robot
`router` service. `phoxal-cli` also starts a host-side `phoxal-local-zenoh`
container from the same upstream `eclipse/zenoh` image. Both containers join
the external bridge network named `phoxal-link`, which gives the simulated robot
and host tools a shared zenoh fabric. Only `phoxal-local-zenoh` publishes a host
port, bound to `127.0.0.1:7447`, so local tools connect through the host-side
router rather than directly to the robot compose service.

## Production Topology

Production runs only the on-robot router. It is configured from the optional
`network` section in `robot.yaml`, which describes upstream zenoh endpoints and
TLS material for deployment. The host-side `phoxal-local-zenoh` container is a
simulation affordance and does not run in production.
