# phoxal-runtime-safety

`phoxal-runtime-safety` is a BLUEPRINT skeleton for immediate safety
authorization.

It consumes localization state, safety-tagged range evidence, safety-tagged
hardware emergency-stop capability state, and operator emergency-stop requests.

It will publish `runtime/safety/authorization`, `runtime/safety/state`, and the
debug products defined by the blueprint contracts.

`EmergencyStop` is absolute priority: when any hardware emergency-stop state or
operator emergency-stop request is engaged, the runtime publishes
`SafetyDecision::EmergencyStop` with zero motion and no protective escape
envelope. Protective `Stop` remains the obstacle/cliff behavior and still keeps
its reverse/rotation escape envelope.

Not in scope: actuator command generation, mission behavior, planner behavior,
or map ownership.
