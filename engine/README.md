# phoxal-engine

Shared runtime process mechanics for framework runtimes and component drivers.

`phoxal-engine` owns the step loop, logical-time source, shutdown polling,
CLI dispatch, runtime config resolution hooks, scenario dispatch, input fan-in
with per-source queue policies, publisher setup, query serving, and recording
test I/O. Runtime crates own domain behavior and narrow resolved config.

## Runtime

Every runtime — including pure query services — implements one trait:

```rust
#[async_trait::async_trait]
pub trait Runtime: Sized + Send {
    const RUNTIME_ID: &'static str;

    type Args: clap::Args + Send + Sync;
    type Config: Send;
    type Input: Send + 'static;

    fn config(args: &Self::Args, common: &RobotRuntimeArgs) -> anyhow::Result<Self::Config>;

    fn clock_period(config: &Self::Config) -> std::time::Duration;

    async fn new(io: &mut Io<Self::Input>, config: Self::Config) -> anyhow::Result<Self>;

    async fn step(&mut self, step: Step, inputs: RuntimeInputs<Self::Input>) -> anyhow::Result<()>;

    async fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn scenarios() -> &'static [ScenarioDescriptor] {
        &[]
    }

    async fn run_scenario(
        name: &str,
        common: &RobotRuntimeArgs,
        args: &Self::Args,
    ) -> anyhow::Result<()>;
}
```

`Runtime::new(...)` declares runtime I/O:

- `io.subscribe(topic, map)` maps a stamped pub/sub payload into `Input` using the
  default `InputPolicy::All`.
- `io.subscribe_with(topic, policy, map)` is the same with an explicit
  [`InputPolicy`](#input-policies).
- `io.subscribe_mirrored(topic, debug_key, map)` /
  `io.subscribe_mirrored_with(topic, debug_key, policy, map)` do the same and
  republish the exact consumed payload to
  `runtime/<runtime-id>/debug/input/<debug_key>` only while a subscriber is
  present.
- `io.serve_request(query, map)` maps a Zenoh query into `Input` with a
  `RequestResponder`; the runtime replies with `responder.reply(&response).await?`
  from inside `step(...)`. Query sources are always `All` — there is no policy
  parameter, because dropping a query would leave the caller hanging.
- `io.publisher::<T>(topic).await?` returns a `Publisher<T>` stored by the
  runtime and used with `.put(&payload).await?` from `step(...)`.
- `io.eager_publisher::<T>(topic).await?` is the explicit command/backbone
  publisher path for topics that must publish regardless of current matches.

`step(...)` does not return publish batches. It consumes `RuntimeInputs<Input>`
and uses stored output handles for pub/sub output.

There is no separate request-runtime model. A pure query service (for example
`runtimes/asset`) is just a `Runtime` that registers `serve_request` and
replies from `step(...)`.

Mirrored subscriptions are input-only debug products. A runtime opts in per
consumed input when Rerun or another operator tool needs to render the same
typed payload the runtime fed into its algorithm. Runtime outputs stay on their
primary published topics and are not mirrored by the harness.

Capability topics come from owner-local API helpers in the unrooted
`topic(component_id, capability_id) -> String` form, such as
`phoxal_component_api::v1::capability::motor::topic(...)`, before they are passed to
`Io`.

## Input policies

`Io` buffers each registered input source independently between logical steps and
applies an `InputPolicy` as messages arrive:

```rust
pub enum InputPolicy {
    All,                               // keep every message since the previous step, in order
    Latest,                            // keep only the most recent; drop older unconsumed
    BoundedDropOldest { max: usize },  // keep at most `max`; drop the oldest when full
}
```

The default for `subscribe` / `subscribe_mirrored` is `All`. Pick the policy from
how the runtime consumes the input — the runtime is the only place that knows
whether older samples still matter:

| Input shape | Policy | Why |
|---|---|---|
| Query / request | `All` (forced) | every request must receive a reply |
| Event/command applied per message — mission/power command, estop, keyframe, revision, heartbeat | `All` | collapsing would drop events or break safety/liveness (a multiplexed heartbeat stream needs every message) |
| Current setpoint the runtime reduces to one value — drive target, follow path, plan goal, motion manual/follow target | `Latest` | only the newest value matters |
| Per-frame sensor where only the freshest frame is processed — perception camera/depth | `Latest` | processing a stale backlog is wasted work |
| Sensor stream a backend re-synchronizes and bounds itself — localize ORB-SLAM3 camera/depth/imu | `All` | the backend pairs RGB-D and caps its own buffers; harness dropping would desync pairs |
| Frame-complete stream — video preview encoder | `All` | dropping frames corrupts the encoded delta stream |

`RuntimeInputs<Input>` is what `step` receives. It iterates the buffered inputs
(`for input in inputs { match input { .. } }`) and carries
`RuntimeInputStats { received, delivered, dropped }` via `inputs.stats()`
(`received == delivered + dropped`). Per-runtime delivered/dropped totals are
logged in the periodic "runtime alive" line.

## Example

```rust
use anyhow::Result;
use phoxal_bus::pubsub::Stamped;
use phoxal_bus::zenoh_typed::TypedSchema;
use phoxal_engine::{EmptyArgs, RobotRuntimeArgs};
use phoxal_engine::step::{InputPolicy, Io, Publisher, Runtime, RuntimeInputs, Step};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct State {
    count: u64,
}

impl TypedSchema for State {
    const SCHEMA_NAME: &'static str = "runtime/example/state";
    const SCHEMA_VERSION: u32 = 1;
}

enum Input {
    Tick(Stamped<State>),
}

struct ExampleRuntime {
    state: Publisher<Stamped<State>>,
}

#[async_trait::async_trait]
impl Runtime for ExampleRuntime {
    const RUNTIME_ID: &'static str = "example";

    type Args = EmptyArgs;
    type Config = ();
    type Input = Input;

    fn config(_args: &Self::Args, _common: &RobotRuntimeArgs) -> Result<Self::Config> {
        Ok(())
    }

    fn clock_period(_config: &Self::Config) -> std::time::Duration {
        std::time::Duration::from_millis(20)
    }

    async fn new(io: &mut Io<Self::Input>, _config: Self::Config) -> Result<Self> {
        io.subscribe_with::<Stamped<State>, _>(
            "runtime/example/input",
            InputPolicy::latest(),
            Input::Tick,
        )
        .await?;

        Ok(Self {
            state: io.publisher::<Stamped<State>>("runtime/example/state").await?,
        })
    }

    async fn step(&mut self, step: Step, inputs: RuntimeInputs<Self::Input>) -> Result<()> {
        let count = inputs.len() as u64 + step.tick.step();
        self.state
            .put(&Stamped::new(step.tick.time_ns(), State { count }))
            .await?;
        Ok(())
    }
}
```

## Process entrypoint

Every runtime binary calls the shared process entrypoint:

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    phoxal_engine::execute::<runtime::ExampleRuntime>().await
}
```

`execute::<R>()` loads `.env`, initializes tracing, parses the common
`RobotRuntimeArgs` plus `R::Args`, and dispatches the uniform `run`, `scenario`,
and `scenarios list` subcommands. The `run` path connects the bus and calls
`RuntimeProcess::new(bus, simulation, R::clock_period(&config)).run::<R>(config)`.

## Recording Tests

Unit tests use `Io::recording()` to create runtime output handles without a live
bus. Recorded publishers clone payloads. Drive `step` directly with
`RuntimeInputs::from(vec![..])` (or `RuntimeInputs::default()` for an empty step).

```rust
use phoxal_engine::step::{Io, Runtime as _, RuntimeInputs};

#[tokio::test]
async fn records_outputs_without_bus() {
    let mut io = Io::recording();
    let mut runtime = ExampleRuntime::new(&mut io, ()).await.expect("new");

    runtime
        .step(
            phoxal_engine::step::Step::new(phoxal_engine::sim_clock::SimulationClock::new(
                0, 0, 1_000, 1_000,
            )),
            RuntimeInputs::default(),
        )
        .await
        .expect("step");

    let states = io.recorded_puts::<Stamped<State>>("runtime/example/state");
    assert_eq!(states.len(), 1);
}
```
