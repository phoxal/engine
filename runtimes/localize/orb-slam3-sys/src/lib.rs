//! GPL-3.0-or-later — see crate-root README.
//!
//! ORB-SLAM3 is licensed under GPL-3.0-or-later. The rest of the robot-framework
//! workspace is dual-licensed per crate: Apache-2.0 on contract/utils/api crates,
//! AGPL-3.0-or-later on runtime/bus/tool crates. This crate always exposes the
//! ORB-SLAM3 C ABI, but links the real ORB-SLAM3 library only when
//! `ORB_SLAM3_DIR` is present at build time and the build script emits the
//! `orb_slam3_linked` cfg. Callers can check [`LINKED`] at runtime.
//!
//! A binary that link-time includes ORB-SLAM3 must be distributed under
//! GPL-3.0-or-later terms. Only AGPL/GPL-compatible runtimes, such as the AGPL
//! localize runtime, may link it. Metadata-only/stub builds link no ORB-SLAM3
//! code and carry no GPL obligation from ORB-SLAM3.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub enum OrbSlam3Handle {}

pub type OrbSlam3Status = u32;
pub type OrbSlam3TrackingState = u32;

pub const OrbSlam3Status_OS3_OK: OrbSlam3Status = 0;
pub const OrbSlam3Status_OS3_ERR_INIT_FAILED: OrbSlam3Status = 1;
pub const OrbSlam3Status_OS3_ERR_TRACK_FAILED: OrbSlam3Status = 2;
pub const OrbSlam3Status_OS3_ERR_INVALID_HANDLE: OrbSlam3Status = 3;
pub const OrbSlam3Status_OS3_ERR_INVALID_INPUT: OrbSlam3Status = 4;
pub const OrbSlam3TrackingState_OS3_TRACKING_NOT_READY: OrbSlam3TrackingState = 0;
pub const OrbSlam3TrackingState_OS3_TRACKING_NO_IMAGES: OrbSlam3TrackingState = 1;
pub const OrbSlam3TrackingState_OS3_TRACKING_NOT_INITIALIZED: OrbSlam3TrackingState = 2;
pub const OrbSlam3TrackingState_OS3_TRACKING_OK: OrbSlam3TrackingState = 3;
pub const OrbSlam3TrackingState_OS3_TRACKING_LOST: OrbSlam3TrackingState = 4;
pub const OrbSlam3TrackingState_OS3_TRACKING_RECENTLY_LOST: OrbSlam3TrackingState = 5;

/// One IMU sample handed to a tracking call. Acceleration is m/s^2, angular
/// velocity is rad/s, and `timestamp_s` is seconds. Layout matches the C
/// `OrbSlam3ImuSample` struct so a slice can cross the FFI boundary directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OrbSlam3ImuSample {
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
    pub gyro_x: f64,
    pub gyro_y: f64,
    pub gyro_z: f64,
    pub timestamp_s: f64,
}

pub const LINKED: bool = cfg!(orb_slam3_linked);

#[cfg(orb_slam3_linked)]
mod linked {
    use super::{OrbSlam3Handle, OrbSlam3ImuSample, OrbSlam3Status, OrbSlam3TrackingState};

    unsafe extern "C" {
        pub fn os3_new_rgbd_inertial(
            vocab_path: *const std::ffi::c_char,
            settings_path: *const std::ffi::c_char,
        ) -> *mut OrbSlam3Handle;
        pub fn os3_new_rgbd(
            vocab_path: *const std::ffi::c_char,
            settings_path: *const std::ffi::c_char,
        ) -> *mut OrbSlam3Handle;
        pub fn os3_destroy(handle: *mut OrbSlam3Handle);
        pub fn os3_track_rgbd(
            handle: *mut OrbSlam3Handle,
            color_bgr: *const u8,
            cols: i32,
            rows: i32,
            color_step_bytes: i32,
            depth_mm: *const u16,
            depth_step_bytes: i32,
            timestamp_s: f64,
            imu_samples: *const OrbSlam3ImuSample,
            imu_count: i32,
            out_translation_m: *mut f64,
            out_rotation_xyzw: *mut f64,
        ) -> OrbSlam3Status;
        pub fn os3_tracking_state(handle: *mut OrbSlam3Handle) -> OrbSlam3TrackingState;
        pub fn os3_poll_map_changed(handle: *mut OrbSlam3Handle) -> i32;
    }
}

#[cfg(orb_slam3_linked)]
pub use linked::*;

#[cfg(not(orb_slam3_linked))]
#[allow(clippy::missing_safety_doc, clippy::too_many_arguments)]
mod stubs {
    use super::{
        OrbSlam3Handle, OrbSlam3ImuSample, OrbSlam3Status, OrbSlam3Status_OS3_ERR_INVALID_HANDLE,
        OrbSlam3TrackingState, OrbSlam3TrackingState_OS3_TRACKING_NOT_READY,
    };

    pub unsafe fn os3_new_rgbd_inertial(
        _vocab_path: *const std::ffi::c_char,
        _settings_path: *const std::ffi::c_char,
    ) -> *mut OrbSlam3Handle {
        std::ptr::null_mut()
    }

    pub unsafe fn os3_new_rgbd(
        _vocab_path: *const std::ffi::c_char,
        _settings_path: *const std::ffi::c_char,
    ) -> *mut OrbSlam3Handle {
        std::ptr::null_mut()
    }

    pub unsafe fn os3_destroy(_handle: *mut OrbSlam3Handle) {}

    pub unsafe fn os3_track_rgbd(
        _handle: *mut OrbSlam3Handle,
        _color_bgr: *const u8,
        _cols: i32,
        _rows: i32,
        _color_step_bytes: i32,
        _depth_mm: *const u16,
        _depth_step_bytes: i32,
        _timestamp_s: f64,
        _imu_samples: *const OrbSlam3ImuSample,
        _imu_count: i32,
        _out_translation_m: *mut f64,
        _out_rotation_xyzw: *mut f64,
    ) -> OrbSlam3Status {
        OrbSlam3Status_OS3_ERR_INVALID_HANDLE
    }

    pub unsafe fn os3_tracking_state(_handle: *mut OrbSlam3Handle) -> OrbSlam3TrackingState {
        OrbSlam3TrackingState_OS3_TRACKING_NOT_READY
    }

    pub unsafe fn os3_poll_map_changed(_handle: *mut OrbSlam3Handle) -> i32 {
        0
    }
}

#[cfg(not(orb_slam3_linked))]
pub use stubs::*;
