#ifndef ORB_SLAM3_RUST_WRAPPER_H
#define ORB_SLAM3_RUST_WRAPPER_H
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OrbSlam3Handle OrbSlam3Handle;

typedef enum {
    OS3_OK = 0,
    OS3_ERR_INIT_FAILED = 1,
    OS3_ERR_TRACK_FAILED = 2,
    OS3_ERR_INVALID_HANDLE = 3,
    OS3_ERR_INVALID_INPUT = 4,
} OrbSlam3Status;

typedef enum {
    OS3_TRACKING_NOT_READY = 0,
    OS3_TRACKING_NO_IMAGES = 1,
    OS3_TRACKING_NOT_INITIALIZED = 2,
    OS3_TRACKING_OK = 3,
    OS3_TRACKING_LOST = 4,
    OS3_TRACKING_RECENTLY_LOST = 5,
} OrbSlam3TrackingState;

/// One IMU sample for a tracking call. Acceleration is m/s^2, angular velocity
/// is rad/s, and `timestamp_s` is seconds.
typedef struct {
    double accel_x;
    double accel_y;
    double accel_z;
    double gyro_x;
    double gyro_y;
    double gyro_z;
    double timestamp_s;
} OrbSlam3ImuSample;

/// Create an RGB-D + Inertial system. `vocab_path` and `settings_path`
/// must point to readable files on disk. Returns NULL on failure.
OrbSlam3Handle* os3_new_rgbd_inertial(const char* vocab_path,
                                      const char* settings_path);

/// Create an RGB-D system without IMU. `vocab_path` and `settings_path`
/// must point to readable files on disk. Returns NULL on failure.
OrbSlam3Handle* os3_new_rgbd(const char* vocab_path,
                             const char* settings_path);

void os3_destroy(OrbSlam3Handle* handle);

/// Track an RGB-D frame. Color is 8-bit BGR, depth is 16-bit mm. The caller
/// passes the IMU samples for this visual frame interval via `imu_samples`
/// (length `imu_count`); the wrapper keeps no IMU state between calls. Pass
/// `imu_samples = NULL` / `imu_count = 0` for non-inertial tracking.
/// Returns OK on success; on success writes pose into out_translation_m[3] and
/// out_rotation_xyzw[4]. Returns ERR_TRACK_FAILED if the tracker rejected
/// the frame (still safe to call again later).
OrbSlam3Status os3_track_rgbd(OrbSlam3Handle* handle,
                               const uint8_t* color_bgr, int32_t cols, int32_t rows,
                               int32_t color_step_bytes,
                               const uint16_t* depth_mm, int32_t depth_step_bytes,
                               double timestamp_s,
                               const OrbSlam3ImuSample* imu_samples, int32_t imu_count,
                               double out_translation_m[3],
                               double out_rotation_xyzw[4]);

OrbSlam3TrackingState os3_tracking_state(OrbSlam3Handle* handle);

/// Returns 1 if ORB-SLAM3 reported a big map change (loop closure / global BA) since the last
/// call, else 0. Returns 0 on a null handle.
int32_t os3_poll_map_changed(OrbSlam3Handle* handle);

#ifdef __cplusplus
}
#endif
#endif
