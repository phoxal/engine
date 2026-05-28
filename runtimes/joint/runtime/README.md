# phoxal-runtime-joint

`phoxal-runtime-joint` resolves joint-targeted encoder capabilities from the
staged robot bundle, subscribes to their component encoder samples, applies the
resolved direction sign, gear ratio, and counts-per-revolution transmission
metadata, and publishes normalized angular joint state on
`runtime/joint/<joint-id>/data`. It emits one joint state for each joint that
receives encoder input in the current step; kinematic inversion, actuator
commands, localization, and position-sensor support are outside this runtime.
