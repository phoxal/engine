# Robot Runtime Video

`phoxal-runtime-video` owns codec camera delivery for operator and web tools. For
v1 it serves one h264 preview stream per camera capability: each
`runtime/video/stream/<stream-id>` packet carries a complete Annex-B h264 access
unit encoded from that stream's camera source. Cameras need no role hint to be
previewed; every color camera capability is streamable, and depth capabilities
remain outside this runtime.

The runtime resolves all cameras at startup and requests a fixed bounded preview
profile from each driver only while that stream is demanded: RGB8, at most 480
px high, at most 10 Hz, preserving the camera aspect ratio. `runtime/video/open`
is a pure query; it returns the deterministic `stream_id` and `StreamFormat` for
the requested `component_id.capability_id` source and does not start, stop, or
mutate runtime state. The request `quality` is accepted for forward
compatibility, but v1 serves this single canonical preview profile.

The codec path is stateful. `video` holds one h264 encoder per camera stream and
forces an IDR keyframe when an operator session starts; the encoder also emits
periodic keyframes so late-joining or recovering operator clients can decode the
stream without waiting indefinitely. This stateful codec path is separate from
the raw Rerun path.

Demand is signaled by liveliness, not by `open`. An operator subscribes to
`runtime/video/stream/<stream-id>` and declares a liveliness token on that same
stream key. While at least one operator token is alive, video declares its own
liveliness token on the camera profile topic
`component/<component-id>/<capability-id>/profile/<profile-id>`, causing the
camera driver to downsample at the source and publish frames. When the final
operator token drops, video drops the camera token and publishes
`StreamEvent::End { reason: Released }`.

Stream events are stamped with simulation logical time from `RuntimeProcess`.
Packets carry `captured_at_ns` from the source camera frame stamp, so capture
time stays distinct from the later video publish time.

This is not the Rerun raw-driver path. Rerun requests raw downsampled camera and
depth driver specs directly so its views stay frame-synced with pose, IMU, and
other runtime products; `video` serves bandwidth-bounded codec previews for
operator and browser delivery.
