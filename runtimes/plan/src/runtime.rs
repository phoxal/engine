use std::time::Duration;

use crate::core::PlanDecision;
use anyhow::Result;
use phoxal_bus::pubsub::Stamped;
use phoxal_engine::clock::Step;
use phoxal_engine::step::{InputPolicy, Io, Publisher, Runtime, RuntimeInputs};
use phoxal_engine::{EmptyArgs, RobotRuntimeArgs};
use phoxal_runtime_localize_api::v1::LocalizationState;
use phoxal_runtime_map_api::v1::MapRevision;
use phoxal_runtime_mission_api::v1::Goal;
use phoxal_runtime_plan_api::v1::{Path, PlanReason, PlanStatus, State, path, state};
use tracing::info;

const CLOCK_PERIOD: Duration = Duration::from_millis(100);

#[derive(Clone, Default)]
pub struct Config {
    clock_period: Option<Duration>,
}

impl Config {
    pub fn clock_period(&self) -> Duration {
        self.clock_period.unwrap_or(CLOCK_PERIOD)
    }
}

pub enum Input {
    Goal(Stamped<Goal>),
    LocalizationState(Stamped<LocalizationState>),
    MapRevision(Stamped<MapRevision>),
}

pub struct PlanRuntime {
    latest_goal: Option<Stamped<Goal>>,
    latest_localize: Option<Stamped<LocalizationState>>,
    latest_map_revision: Option<Stamped<MapRevision>>,
    last_logged_state: Option<PlanLogKey>,
    path_publisher: Publisher<Stamped<Path>>,
    state_publisher: Publisher<Stamped<State>>,
}

type PlanLogKey = (PlanStatus, Option<PlanReason>);

#[async_trait::async_trait]
impl Runtime for PlanRuntime {
    const RUNTIME_ID: &'static str = "plan";

    type Args = EmptyArgs;
    type Config = Config;
    type Input = Input;

    fn config(_args: &Self::Args, _common: &RobotRuntimeArgs) -> Result<Self::Config> {
        Ok(Config::default())
    }

    fn clock_period(config: &Self::Config) -> Duration {
        config.clock_period()
    }

    async fn new(io: &mut Io<Self::Input>, _config: Self::Config) -> Result<Self> {
        io.subscribe_with::<Stamped<Goal>, _>(
            phoxal_runtime_mission_api::v1::goal::TOPIC,
            InputPolicy::latest(),
            Input::Goal,
        )
        .await?;
        io.subscribe::<Stamped<LocalizationState>, _>(
            phoxal_runtime_localize_api::v1::state::TOPIC,
            Input::LocalizationState,
        )
        .await?;
        io.subscribe::<Stamped<MapRevision>, _>(
            phoxal_runtime_map_api::v1::revision::TOPIC,
            Input::MapRevision,
        )
        .await?;

        let path_publisher = io.publisher::<Stamped<Path>>(path::TOPIC).await?;
        let state_publisher = io.publisher::<Stamped<State>>(state::TOPIC).await?;

        Ok(Self {
            latest_goal: None,
            latest_localize: None,
            latest_map_revision: None,
            last_logged_state: None,
            path_publisher,
            state_publisher,
        })
    }

    async fn step(&mut self, step: Step, inputs: RuntimeInputs<Self::Input>) -> Result<()> {
        for input in inputs {
            match input {
                Input::Goal(sample) => self.latest_goal = Some(sample),
                Input::LocalizationState(sample) => self.latest_localize = Some(sample),
                Input::MapRevision(sample) => self.latest_map_revision = Some(sample),
            }
        }

        // Simple receding horizon: every step republishes a fresh path from the
        // latest pose to the latest goal instead of caching a long-lived plan.
        let decision = PlanDecision::decide(
            self.latest_goal.as_ref().map(|sample| &sample.data),
            self.latest_localize.as_ref().map(|sample| &sample.data),
            self.latest_map_revision.as_ref().map(|sample| &sample.data),
        );
        let timestamp_ns = step.tick.time_ns();
        let (state, path) = decision.outputs(self.latest_goal.as_ref().map(|sample| &sample.data));

        if let Some(path) = path {
            self.path_publisher
                .put(&Stamped::new(timestamp_ns, path))
                .await?;
        }
        let logged = (state.status, state.reason);
        if self.last_logged_state != Some(logged) {
            info!(
                status = ?state.status,
                reason = ?state.reason,
                "plan state changed"
            );
            self.last_logged_state = Some(logged);
        }
        self.state_publisher
            .put(&Stamped::new(timestamp_ns, state))
            .await?;

        Ok(())
    }

    fn scenarios() -> &'static [phoxal_engine::step::ScenarioDescriptor] {
        crate::scenarios::SCENARIOS
    }

    async fn run_scenario(name: &str, common: &RobotRuntimeArgs, _args: &Self::Args) -> Result<()> {
        crate::scenarios::run(name, common).await
    }
}
