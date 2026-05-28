# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.0-dev](https://github.com/phoxal/framework/releases/tag/phoxal-engine-v0.0.0-dev) - 2026-05-28

### Added

- *(utils-robot)* single Robot struct + new robot.yaml schema

### Fixed

- *(tests)* relocate fixture/ into framework; fix post-flatten paths

### Other

- *(workspace)* drop utils- prefix; merge scenario crates; structure runtimes/<name>/{api,runtime}/
- *(workspace)* drop phoxal-simulator-api workspace dep
- *(engine)* own SimulationClock; drop engine→simulator-api dep edge
- *(framework)* delete dead RuntimeBudget; adopt v1 dispatcher in utils-robot; sweep dead code
- *(release)* configure release-plz + cargo-dist + publish flags (dry-run)
- *(api)* introduce pub mod v1 in every phoxal-*-api crate
- *(engine)* fold phoxal-utils-conventions into phoxal-engine
- *(workspace)* carve members into future-repo subdirs

### Breaking

- Unified the runtime model into a single `phoxal_engine::step::Runtime`
  trait run through `RuntimeProcess`. The `StepRuntime`/`RequestRuntime` split and
  `RequestProcess`/`RequestEndpoint` are removed; a pure query service is now a
  `Runtime` that registers `Io::serve_request(...)` and replies from `step(...)`.
  `step(...)` receives `RuntimeInputs<Input>` instead of `Vec<Input>`.
- Input fan-in is now policy-based per source. `Io::subscribe(...)` defaults to
  `InputPolicy::All`; `Io::subscribe_with(topic, policy, map)` selects `Latest`
  or `BoundedDropOldest`. Query sources are always `All`. `RuntimeInputs` carries
  `RuntimeInputStats { received, delivered, dropped }`, logged per runtime.

### Changed

- Simplified simulation supervisor synchronization to consume only
  `simulation/clock` and removed ready/reset/step-ack protocol behavior.

## [0.1.0](https://github.com/phoxal/engine/releases/tag/v0.1.0) - 2026-03-14

### Fixed

- fix manifest

### Other

- xtask uses tmp files
- Refactor to robot_model and add module exclusion features
- Robot-Model Provisioning Plan
- models!!!
- improvements
- Merge pull request #706 from jBernavaPrah/refactor-phoxal-engine-10934194409128922556
- *(phoxal-engine)* apply requested improvements to crate
- improvements
- improvements
- improve
- improve
- wip
- manifest + xtask
- wip
- added systemd with steroids
- Merge utils/bus into robot-utils/bus and strictly use client Zenoh mode
- rename robot-binary to robot-runtime and implement dynamic discovery
