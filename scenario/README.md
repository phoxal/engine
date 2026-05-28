# phoxal-scenario

`phoxal-scenario` owns serializable scenario spec and report data:
`ScenarioSpec`, `WebotsWorld`, `ScenarioOutcome`, `ScenarioResult`, and
`ScenarioReport`.

It is a foundation crate with no runtime, simulator, bus, or harness
dependencies. Orchestration tools use it when they only need to read, write, or
format scenario metadata and reports. The live scenario harness remains in
`phoxal-scenario`, which re-exports these modules for harness consumers.
