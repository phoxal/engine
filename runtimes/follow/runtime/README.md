# phoxal-runtime-follow

`phoxal-runtime-follow` is a BLUEPRINT skeleton for turning an active path or
trajectory into short-horizon motion requests.

It consumes plan path, localization state, traversability, and safety
authorization.

It publishes `runtime/follow/target`, `runtime/follow/state`, and debug
products for tracking error, candidates, costs, and revision inputs.
`runtime/follow/state` carries a typed `FollowReason` for pause/refusal/arrival
causes.

Not in scope: goal choice, path planning, actuator commands, or bypassing
safety authorization.
