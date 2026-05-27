# phoxal-runtime-motion

`phoxal-runtime-motion` is a BLUEPRINT skeleton for motion authority arbitration.

It will select between manual, follower, mission stop, recovery, emergency stop,
and degraded-mode command sources while enforcing freshness and priority.

Primary products: `runtime/motion/state`, `runtime/drive/target`, and debug
products for arbitration and source freshness. `runtime/motion/state` carries a
typed `MotionReason` for primary arbitration causes; debug products keep
human-readable strings.

Not in scope: kinematic inversion, actuator command generation, safety
authorization, or mission behavior.
