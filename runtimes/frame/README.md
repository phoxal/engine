# phoxal-runtime-frame

`phoxal-runtime-frame` loads the robot URDF structure, publishes the revisioned frame tree, publishes cacheable static transforms once at startup, subscribes to non-fixed joint state topics, buffers dynamic child-link transforms in bounded per-frame history, publishes updated dynamic transforms on `runtime/frame/<frame-id>/data`, and answers `runtime/frame/lookup` requests by composing static edges with the nearest timestamped dynamic samples.
