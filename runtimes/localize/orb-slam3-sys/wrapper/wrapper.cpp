#include "wrapper.h"

#include <cmath>
#include <memory>
#include <utility>
#include <vector>

#include <opencv2/core.hpp>

#include "ImuTypes.h"
#include "System.h"

struct OrbSlam3Handle {
    explicit OrbSlam3Handle(std::unique_ptr<ORB_SLAM3::System> system)
        : system(std::move(system)) {}

    std::unique_ptr<ORB_SLAM3::System> system;
};

namespace {

bool invalid_string(const char* value) {
    return value == nullptr || value[0] == '\0';
}

bool invalid_frame(const uint8_t* color_bgr,
                   int32_t cols,
                   int32_t rows,
                   int32_t color_step_bytes,
                   const uint16_t* depth_mm,
                   int32_t depth_step_bytes,
                   double* out_translation_m,
                   double* out_rotation_xyzw) {
    return color_bgr == nullptr || depth_mm == nullptr ||
           out_translation_m == nullptr || out_rotation_xyzw == nullptr ||
           cols <= 0 || rows <= 0 || color_step_bytes < cols * 3 ||
           depth_step_bytes < cols * static_cast<int32_t>(sizeof(uint16_t));
}

OrbSlam3TrackingState map_tracking_state(int state) {
    switch (state) {
        case -1:
            return OS3_TRACKING_NOT_READY;
        case 0:
            return OS3_TRACKING_NO_IMAGES;
        case 1:
            return OS3_TRACKING_NOT_INITIALIZED;
        case 2:
            return OS3_TRACKING_OK;
        case 3:
            return OS3_TRACKING_RECENTLY_LOST;
        case 4:
            return OS3_TRACKING_LOST;
        default:
            return OS3_TRACKING_LOST;
    }
}

}  // namespace

extern "C" OrbSlam3Handle* os3_new_rgbd_inertial(const char* vocab_path,
                                                  const char* settings_path) {
    if (invalid_string(vocab_path) || invalid_string(settings_path)) {
        return nullptr;
    }

    try {
        auto system = std::make_unique<ORB_SLAM3::System>(
            vocab_path, settings_path, ORB_SLAM3::System::IMU_RGBD, false);
        return new OrbSlam3Handle(std::move(system));
    } catch (...) {
        return nullptr;
    }
}

extern "C" OrbSlam3Handle* os3_new_rgbd(const char* vocab_path,
                                         const char* settings_path) {
    if (invalid_string(vocab_path) || invalid_string(settings_path)) {
        return nullptr;
    }

    try {
        auto system = std::make_unique<ORB_SLAM3::System>(
            vocab_path, settings_path, ORB_SLAM3::System::RGBD, false);
        return new OrbSlam3Handle(std::move(system));
    } catch (...) {
        return nullptr;
    }
}

extern "C" void os3_destroy(OrbSlam3Handle* handle) {
    if (handle == nullptr) {
        return;
    }

    try {
        handle->system->Shutdown();
    } catch (...) {
    }
    delete handle;
}

extern "C" OrbSlam3Status os3_track_rgbd(OrbSlam3Handle* handle,
                                          const uint8_t* color_bgr,
                                          int32_t cols,
                                          int32_t rows,
                                          int32_t color_step_bytes,
                                          const uint16_t* depth_mm,
                                          int32_t depth_step_bytes,
                                          double timestamp_s,
                                          const OrbSlam3ImuSample* imu_samples,
                                          int32_t imu_count,
                                          double out_translation_m[3],
                                          double out_rotation_xyzw[4]) {
    if (handle == nullptr) {
        return OS3_ERR_INVALID_HANDLE;
    }
    if (invalid_frame(color_bgr, cols, rows, color_step_bytes, depth_mm, depth_step_bytes,
                      out_translation_m, out_rotation_xyzw) ||
        !std::isfinite(timestamp_s) || imu_count < 0 ||
        (imu_count > 0 && imu_samples == nullptr)) {
        return OS3_ERR_INVALID_INPUT;
    }

    try {
        cv::Mat color(rows, cols, CV_8UC3, const_cast<uint8_t*>(color_bgr), color_step_bytes);
        cv::Mat depth(rows, cols, CV_16UC1, const_cast<uint16_t*>(depth_mm), depth_step_bytes);
        std::vector<ORB_SLAM3::IMU::Point> imu;
        imu.reserve(static_cast<size_t>(imu_count));
        for (int32_t i = 0; i < imu_count; ++i) {
            const OrbSlam3ImuSample& sample = imu_samples[i];
            if (!std::isfinite(sample.accel_x) || !std::isfinite(sample.accel_y) ||
                !std::isfinite(sample.accel_z) || !std::isfinite(sample.gyro_x) ||
                !std::isfinite(sample.gyro_y) || !std::isfinite(sample.gyro_z) ||
                !std::isfinite(sample.timestamp_s)) {
                return OS3_ERR_INVALID_INPUT;
            }
            imu.emplace_back(
                static_cast<float>(sample.accel_x),
                static_cast<float>(sample.accel_y),
                static_cast<float>(sample.accel_z),
                static_cast<float>(sample.gyro_x),
                static_cast<float>(sample.gyro_y),
                static_cast<float>(sample.gyro_z),
                sample.timestamp_s);
        }

        Sophus::SE3f pose = handle->system->TrackRGBD(color, depth, timestamp_s, imu);
        if (pose.matrix().hasNaN()) {
            return OS3_ERR_TRACK_FAILED;
        }

        const Eigen::Vector3f translation = pose.translation();
        const Eigen::Quaternionf rotation = pose.unit_quaternion();
        out_translation_m[0] = static_cast<double>(translation.x());
        out_translation_m[1] = static_cast<double>(translation.y());
        out_translation_m[2] = static_cast<double>(translation.z());
        out_rotation_xyzw[0] = static_cast<double>(rotation.x());
        out_rotation_xyzw[1] = static_cast<double>(rotation.y());
        out_rotation_xyzw[2] = static_cast<double>(rotation.z());
        out_rotation_xyzw[3] = static_cast<double>(rotation.w());
        return OS3_OK;
    } catch (...) {
        return OS3_ERR_TRACK_FAILED;
    }
}

extern "C" OrbSlam3TrackingState os3_tracking_state(OrbSlam3Handle* handle) {
    if (handle == nullptr) {
        return OS3_TRACKING_NOT_READY;
    }

    try {
        return map_tracking_state(handle->system->GetTrackingState());
    } catch (...) {
        return OS3_TRACKING_LOST;
    }
}

extern "C" int32_t os3_poll_map_changed(OrbSlam3Handle* handle) {
    if (handle == nullptr) {
        return 0;
    }
    try {
        return handle->system->MapChanged() ? 1 : 0;
    } catch (...) {
        return 0;
    }
}
