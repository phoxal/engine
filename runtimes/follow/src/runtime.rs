use std::time::Duration;

use crate::core::FollowDecision;
use anyhow::Result;
use phoxal_bus::pubsub::Stamped;
use phoxal_engine::clock::Step;
use phoxal_engine::step::{InputPolicy, Io, Publisher, Runtime, RuntimeInputs};
use phoxal_engine::{EmptyArgs, RobotRuntimeArgs};
use phoxal_runtime_follow_api::v1::{FollowReason, FollowStatus, State, Target, state, target};
use phoxal_runtime_localize_api::v1::LocalizationState;
use phoxal_runtime_plan_api::v1::{Path, path as plan_path};
use tracing::info;

const CLOCK_PERIOD: Duration = Duration::from_millis(50);

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
    Path(Stamped<Path>),
    LocalizationState(Stamped<LocalizationState>),
}

pub struct FollowRuntime {
    latest_path: Option<Stamped<Path>>,
    latest_localize: Option<Stamped<LocalizationState>>,
    last_logged_state: Option<FollowLogKey>,
    target_publisher: Publisher<Stamped<Target>>,
    state_publisher: Publisher<Stamped<State>>,
}

type FollowLogKey = (FollowStatus, Option<FollowReason>);

#[async_trait::async_trait]
impl Runtime for FollowRuntime {
    const RUNTIME_ID: &'static str = "follow";

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
        io.subscribe_with::<Stamped<Path>, _>(plan_path::TOPIC, InputPolicy::latest(), Input::Path)
            .await?;
        io.subscribe::<Stamped<LocalizationState>, _>(
            phoxal_runtime_localize_api::v1::state::TOPIC,
            Input::LocalizationState,
        )
        .await?;

        let target_publisher = io.publisher::<Stamped<Target>>(target::TOPIC).await?;
        let state_publisher = io.publisher::<Stamped<State>>(state::TOPIC).await?;

        Ok(Self {
            latest_path: None,
            latest_localize: None,
            last_logged_state: None,
            target_publisher,
            state_publisher,
        })
    }

    async fn step(&mut self, step: Step, inputs: RuntimeInputs<Self::Input>) -> Result<()> {
        for input in inputs {
            match input {
                Input::Path(sample) => self.latest_path = Some(sample),
                Input::LocalizationState(sample) => self.latest_localize = Some(sample),
            }
        }

        let decision = FollowDecision::decide(
            self.latest_path.as_ref().map(|sample| &sample.data),
            self.latest_localize.as_ref().map(|sample| &sample.data),
        );
        let (state, target) =
            decision.outputs(self.latest_path.as_ref().map(|sample| &sample.data));
        let timestamp_ns = step.tick.time_ns();

        self.target_publisher
            .put(&Stamped::new(timestamp_ns, target))
            .await?;
        let logged = (state.status, state.reason);
        if self.last_logged_state != Some(logged) {
            info!(
                status = ?state.status,
                reason = ?state.reason,
                "follow state changed"
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
