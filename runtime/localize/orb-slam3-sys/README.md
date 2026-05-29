# phoxal-runtime-localize-orb-slam3-sys

Thin C-ABI bindings for the ORB-SLAM3 RGB-D inertial backend.

ORB-SLAM3 is GPL-3.0-or-later. This crate always exposes the small ORB-SLAM3 C
ABI to Rust callers. When `ORB_SLAM3_DIR` is set at build time, the build script
compiles the C++ wrapper, links ORB-SLAM3, and sets the `orb_slam3_linked` cfg.
When it is not set, the crate builds metadata-only Rust stubs and links no
ORB-SLAM3 code. Callers can check `phoxal_runtime_localize_orb_slam3_sys::LINKED` at runtime.

Any binary that link-time includes ORB-SLAM3 must be distributed under
GPL-3.0-or-later terms. Only AGPL/GPL-compatible runtimes, such as the AGPL
localize runtime, link it. Metadata-only/stub builds carry no GPL obligation
from ORB-SLAM3.

## Environment

- `ORB_SLAM3_DIR`: ORB-SLAM3 install prefix. The Docker image uses
  `/ORB_SLAM3` and must contain `lib/libORB_SLAM3.so`, `include/`,
  `Thirdparty/DBoW2/lib`, and `Thirdparty/g2o/lib`.
- `PANGOLIN_DIR`: Pangolin install prefix. Defaults to `/deploy` for
  `tingoose/orb-slam3:latest`.

## Docker Build

```sh
docker build -f runtime/localize/phoxal-runtime-localize-orb-slam3-sys/Dockerfile .
```

macOS native builds without `ORB_SLAM3_DIR` do not require ORB-SLAM3 and
continue to use the Rust stubs.
