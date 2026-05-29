use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CameraIntrinsics {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) fx: f64,
    pub(crate) fy: f64,
    pub(crate) cx: f64,
    pub(crate) cy: f64,
}

impl CameraIntrinsics {
    /// Pinhole intrinsics from image size + horizontal field of view, assuming
    /// square pixels (fy == fx) and a centered principal point.
    pub(crate) fn from_horizontal_fov(
        width: u32,
        height: u32,
        horizontal_fov_rad: f64,
    ) -> Result<Self> {
        if width == 0 || height == 0 {
            bail!("camera intrinsics require non-zero width/height");
        }
        if !horizontal_fov_rad.is_finite()
            || horizontal_fov_rad <= 0.0
            || horizontal_fov_rad >= std::f64::consts::PI
        {
            bail!("camera horizontal FOV must be in (0, PI), got {horizontal_fov_rad}");
        }

        let fx = (f64::from(width) / 2.0) / (horizontal_fov_rad / 2.0).tan();
        Ok(Self {
            width,
            height,
            fx,
            fy: fx,
            cx: f64::from(width) / 2.0,
            cy: f64::from(height) / 2.0,
        })
    }
}

/// Render an ORB-SLAM3 RGB-D + Inertial settings YAML from resolved intrinsics.
///
/// IMU↔camera extrinsics are derived by the selector from the model frame tree.
/// IMU noise + ORB-extractor params are sensible defaults to be refined during
/// tracking tuning.
pub(crate) fn render_rgbd_inertial_settings(
    intr: &CameraIntrinsics,
    depth_map_factor: f64,
    camera_fps: f64,
    imu_frequency_hz: f64,
    imu_to_camera_optical: ([f64; 3], [[f64; 3]; 3]),
) -> String {
    let (translation, rotation) = imu_to_camera_optical;
    let imu_t_b_c1_data = [
        yaml_matrix_value(rotation[0][0]),
        yaml_matrix_value(rotation[0][1]),
        yaml_matrix_value(rotation[0][2]),
        yaml_matrix_value(translation[0]),
        yaml_matrix_value(rotation[1][0]),
        yaml_matrix_value(rotation[1][1]),
        yaml_matrix_value(rotation[1][2]),
        yaml_matrix_value(translation[1]),
        yaml_matrix_value(rotation[2][0]),
        yaml_matrix_value(rotation[2][1]),
        yaml_matrix_value(rotation[2][2]),
        yaml_matrix_value(translation[2]),
    ];

    format!(
        r#"%YAML:1.0
File.version: "1.0"
Camera.type: "PinHole"
Camera1.fx: {}
Camera1.fy: {}
Camera1.cx: {}
Camera1.cy: {}
Camera1.k1: 0.0
Camera1.k2: 0.0
Camera1.p1: 0.0
Camera1.p2: 0.0
Camera.width: {}
Camera.height: {}
Camera.fps: {}
Camera.RGB: 0
Stereo.ThDepth: 40.0
Stereo.b: 0.05
RGBD.DepthMapFactor: {}
IMU.T_b_c1: !!opencv-matrix
   rows: 4
   cols: 4
   dt: f
   data: [{}, {}, {}, {},
          {}, {}, {}, {},
          {}, {}, {}, {},
          0.0, 0.0, 0.0, 1.0]
IMU.InsertKFsWhenLost: 0
IMU.NoiseGyro: 1e-2
IMU.NoiseAcc: 1e-1
IMU.GyroWalk: 1e-6
IMU.AccWalk: 1e-4
IMU.Frequency: {}
ORBextractor.nFeatures: 2500
ORBextractor.scaleFactor: 1.2
ORBextractor.nLevels: 8
ORBextractor.iniThFAST: 10
ORBextractor.minThFAST: 4
Viewer.KeyFrameSize: 0.05
Viewer.KeyFrameLineWidth: 1.0
Viewer.GraphLineWidth: 0.9
Viewer.PointSize: 2.0
Viewer.CameraSize: 0.08
Viewer.CameraLineWidth: 3.0
Viewer.ViewpointX: 0.0
Viewer.ViewpointY: -0.7
Viewer.ViewpointZ: -3.5
Viewer.ViewpointF: 500.0
"#,
        yaml_decimal(intr.fx),
        yaml_decimal(intr.fy),
        yaml_decimal(intr.cx),
        yaml_decimal(intr.cy),
        intr.width,
        intr.height,
        yaml_fps(camera_fps),
        yaml_decimal(depth_map_factor),
        imu_t_b_c1_data[0],
        imu_t_b_c1_data[1],
        imu_t_b_c1_data[2],
        imu_t_b_c1_data[3],
        imu_t_b_c1_data[4],
        imu_t_b_c1_data[5],
        imu_t_b_c1_data[6],
        imu_t_b_c1_data[7],
        imu_t_b_c1_data[8],
        imu_t_b_c1_data[9],
        imu_t_b_c1_data[10],
        imu_t_b_c1_data[11],
        yaml_one_decimal(imu_frequency_hz),
    )
}

/// Render an ORB-SLAM3 RGB-D settings YAML from resolved intrinsics.
pub(crate) fn render_rgbd_settings(
    intr: &CameraIntrinsics,
    depth_map_factor: f64,
    camera_fps: f64,
) -> String {
    format!(
        r#"%YAML:1.0
File.version: "1.0"
Camera.type: "PinHole"
Camera1.fx: {}
Camera1.fy: {}
Camera1.cx: {}
Camera1.cy: {}
Camera1.k1: 0.0
Camera1.k2: 0.0
Camera1.p1: 0.0
Camera1.p2: 0.0
Camera.width: {}
Camera.height: {}
Camera.fps: {}
Camera.RGB: 0
Stereo.ThDepth: 40.0
Stereo.b: 0.05
RGBD.DepthMapFactor: {}
ORBextractor.nFeatures: 2500
ORBextractor.scaleFactor: 1.2
ORBextractor.nLevels: 8
ORBextractor.iniThFAST: 10
ORBextractor.minThFAST: 4
Viewer.KeyFrameSize: 0.05
Viewer.KeyFrameLineWidth: 1.0
Viewer.GraphLineWidth: 0.9
Viewer.PointSize: 2.0
Viewer.CameraSize: 0.08
Viewer.CameraLineWidth: 3.0
Viewer.ViewpointX: 0.0
Viewer.ViewpointY: -0.7
Viewer.ViewpointZ: -3.5
Viewer.ViewpointF: 500.0
"#,
        yaml_decimal(intr.fx),
        yaml_decimal(intr.fy),
        yaml_decimal(intr.cx),
        yaml_decimal(intr.cy),
        intr.width,
        intr.height,
        yaml_fps(camera_fps),
        yaml_decimal(depth_map_factor),
    )
}

fn yaml_decimal(value: f64) -> String {
    format!("{:.1}", value.round())
}

fn yaml_one_decimal(value: f64) -> String {
    format!("{value:.1}")
}

fn yaml_matrix_value(value: f64) -> String {
    format!("{value:.9}")
}

fn yaml_fps(value: f64) -> String {
    let rounded = value.round();
    if (value - rounded).abs() <= f64::EPSILON {
        format!("{rounded:.0}")
    } else {
        format!("{value:.1}")
    }
}

#[cfg(test)]
mod tests {
    use super::{CameraIntrinsics, render_rgbd_inertial_settings, render_rgbd_settings};

    const IDENTITY_EXT: ([f64; 3], [[f64; 3]; 3]) = (
        [0.0, 0.0, 0.0],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    );

    #[test]
    fn intrinsics_from_oak_d_lite_rgb_fov() {
        let intrinsics = match CameraIntrinsics::from_horizontal_fov(640, 480, 1.204277) {
            Ok(intrinsics) => intrinsics,
            Err(error) => panic!("intrinsics should derive from oak_d_lite RGB FOV: {error:#}"),
        };

        assert!((intrinsics.fx - 466.0).abs() < 1.0);
        assert_eq!(intrinsics.fy, intrinsics.fx);
        assert_eq!(intrinsics.cx, 320.0);
        assert_eq!(intrinsics.cy, 240.0);
    }

    #[test]
    fn intrinsics_reject_zero_dimensions() {
        assert!(CameraIntrinsics::from_horizontal_fov(0, 480, 1.204277).is_err());
        assert!(CameraIntrinsics::from_horizontal_fov(640, 0, 1.204277).is_err());
    }

    #[test]
    fn intrinsics_reject_out_of_range_fov() {
        assert!(CameraIntrinsics::from_horizontal_fov(640, 480, 0.0).is_err());
        assert!(CameraIntrinsics::from_horizontal_fov(640, 480, std::f64::consts::PI).is_err());
        assert!(CameraIntrinsics::from_horizontal_fov(640, 480, f64::NAN).is_err());
    }

    #[test]
    fn rendered_settings_contain_resolved_intrinsics() {
        let intrinsics = match CameraIntrinsics::from_horizontal_fov(640, 480, 1.204277) {
            Ok(intrinsics) => intrinsics,
            Err(error) => panic!("intrinsics should derive from oak_d_lite RGB FOV: {error:#}"),
        };
        let settings = render_rgbd_inertial_settings(&intrinsics, 1000.0, 15.0, 50.0, IDENTITY_EXT);

        assert!(settings.contains("Camera1.cx: 320.0"));
        assert!(settings.contains("Camera1.cy: 240.0"));
        assert!(settings.contains("Camera.width: 640"));
        assert!(settings.contains("Camera.height: 480"));
        assert!(settings.contains("RGBD.DepthMapFactor: 1000.0"));
        assert!(
            settings
                .lines()
                .any(|line| line.starts_with("Camera1.fx: 466"))
        );
    }

    #[test]
    fn rendered_settings_reflect_feed_rates_and_bgr() {
        let intrinsics = CameraIntrinsics {
            width: 640,
            height: 480,
            fx: 466.0,
            fy: 466.0,
            cx: 320.0,
            cy: 240.0,
        };
        let settings = render_rgbd_inertial_settings(&intrinsics, 1000.0, 15.0, 50.0, IDENTITY_EXT);

        assert!(settings.contains("Camera.fps: 15"));
        assert!(settings.contains("IMU.Frequency: 50.0"));
        assert!(settings.contains("Camera.RGB: 0"));
    }

    #[test]
    fn rendered_settings_emit_imu_t_b_c1_from_extrinsic() {
        let intrinsics = CameraIntrinsics {
            width: 640,
            height: 480,
            fx: 466.0,
            fy: 466.0,
            cx: 320.0,
            cy: 240.0,
        };
        let extrinsic = (
            [0.01, 0.02, 0.03],
            [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        );

        let settings = render_rgbd_inertial_settings(&intrinsics, 1000.0, 15.0, 50.0, extrinsic);

        assert!(
            settings.contains("data: [0.000000000, -1.000000000, 0.000000000, 0.010000000,"),
            "first row of IMU.T_b_c1 should reflect R00..R02 and tx; got:\n{settings}"
        );
        assert!(
            settings.contains("0.0, 0.0, 0.0, 1.0]"),
            "last row of IMU.T_b_c1 must be [0, 0, 0, 1]; got:\n{settings}"
        );
        assert!(
            !settings.contains("1.0, 0.0, 0.0, 0.0,\n          0.0, 1.0, 0.0, 0.0,"),
            "old identity IMU.T_b_c1 block must not be present"
        );
    }

    #[test]
    fn rendered_rgbd_settings_contain_intrinsics_without_imu() {
        let intrinsics = CameraIntrinsics {
            width: 640,
            height: 480,
            fx: 466.0,
            fy: 466.0,
            cx: 320.0,
            cy: 240.0,
        };
        let settings = render_rgbd_settings(&intrinsics, 1000.0, 15.0);

        assert!(settings.contains("Camera1.fx: 466.0"));
        assert!(settings.contains("Camera1.fy: 466.0"));
        assert!(settings.contains("Camera1.cx: 320.0"));
        assert!(settings.contains("Camera1.cy: 240.0"));
        assert!(settings.contains("Camera.width: 640"));
        assert!(settings.contains("Camera.height: 480"));
        assert!(settings.contains("RGBD.DepthMapFactor: 1000.0"));
        assert!(!settings.contains("IMU."));
    }
}
