//! Pure pose math shared by localization backends. No FFI, always compiled, so it is
//! unit-tested without the `orb_slam3` feature (whose native libraries are unavailable locally).

pub(crate) const CAMERA_OPTICAL_FROM_LINK: [f64; 4] = [-0.5, 0.5, -0.5, 0.5];

/// Invert a rigid pose given as (translation, unit quaternion `[x, y, z, w]`).
///
/// ORB-SLAM3 `TrackRGBD` returns `Tcw` (the world->camera transform): its translation is the
/// world origin expressed in camera coordinates, not the camera position in the world.
/// Inverting yields `Twc` (the camera pose in the world) -- translation is the camera position,
/// rotation is the camera orientation -- which is the trajectory we publish.
pub(crate) fn invert_pose(
    translation_m: [f64; 3],
    rotation_xyzw: [f64; 4],
) -> ([f64; 3], [f64; 4]) {
    let [qx, qy, qz, qw] = rotation_xyzw;
    // The inverse of a unit quaternion is its conjugate.
    let inverse_rotation = [-qx, -qy, -qz, qw];
    let rotated = rotate_vector_by_quaternion(inverse_rotation, translation_m);
    let inverse_translation = [-rotated[0], -rotated[1], -rotated[2]];
    (inverse_translation, inverse_rotation)
}

pub(crate) fn camera_optical_to_base_extrinsic(
    link_translation_m: [f64; 3],
    link_rotation_xyzw: [f64; 4],
) -> ([f64; 3], [f64; 4]) {
    let (base_from_camera_translation_m, base_from_camera_rotation_xyzw) = compose_poses(
        link_translation_m,
        link_rotation_xyzw,
        [0.0, 0.0, 0.0],
        CAMERA_OPTICAL_FROM_LINK,
    );
    invert_pose(
        base_from_camera_translation_m,
        base_from_camera_rotation_xyzw,
    )
}

pub(crate) fn compose_poses(
    a_translation_m: [f64; 3],
    a_rotation_xyzw: [f64; 4],
    b_translation_m: [f64; 3],
    b_rotation_xyzw: [f64; 4],
) -> ([f64; 3], [f64; 4]) {
    let rotated_translation = rotate_vector_by_quaternion(a_rotation_xyzw, b_translation_m);
    (
        [
            a_translation_m[0] + rotated_translation[0],
            a_translation_m[1] + rotated_translation[1],
            a_translation_m[2] + rotated_translation[2],
        ],
        quat_mul(a_rotation_xyzw, b_rotation_xyzw),
    )
}

/// Convert a unit quaternion `[x, y, z, w]` into its row-major 3x3 rotation
/// matrix. The matrix maps vectors expressed in the source frame to the same
/// vectors expressed in the target frame.
pub(crate) fn rotation_matrix_from_quaternion(q: [f64; 4]) -> [[f64; 3]; 3] {
    let [x, y, z, w] = q;
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    [
        [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy)],
        [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx)],
        [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy)],
    ]
}

pub(crate) fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    let [ax, ay, az, aw] = a;
    let [bx, by, bz, bw] = b;
    [
        aw * bx + ax * bw + ay * bz - az * by,
        aw * by - ax * bz + ay * bw + az * bx,
        aw * bz + ax * by - ay * bx + az * bw,
        aw * bw - ax * bx - ay * by - az * bz,
    ]
}

/// Rotate a 3-vector by a unit quaternion `[x, y, z, w]` (v' = q * v * q^-1).
fn rotate_vector_by_quaternion(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
    let axis = [q[0], q[1], q[2]];
    let w = q[3];
    let t = scale(cross(axis, v), 2.0);
    let w_t = scale(t, w);
    let axis_t = cross(axis, t);
    [
        v[0] + w_t[0] + axis_t[0],
        v[1] + w_t[1] + axis_t[1],
        v[2] + w_t[2] + axis_t[2],
    ]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[cfg(test)]
mod tests {
    use super::{
        CAMERA_OPTICAL_FROM_LINK, camera_optical_to_base_extrinsic, compose_poses, invert_pose,
        quat_mul, rotate_vector_by_quaternion, rotation_matrix_from_quaternion,
    };

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }

    fn assert_array_close(actual: [f64; 3], expected: [f64; 3]) {
        for (actual, expected) in actual.into_iter().zip(expected) {
            close(actual, expected);
        }
    }

    fn assert_quat_close(actual: [f64; 4], expected: [f64; 4]) {
        for (actual, expected) in actual.into_iter().zip(expected) {
            close(actual, expected);
        }
    }

    fn yaw_quaternion(rad: f64) -> [f64; 4] {
        let half_angle = rad / 2.0;
        [0.0, 0.0, half_angle.sin(), half_angle.cos()]
    }

    #[test]
    fn inverting_identity_is_identity() {
        let (t, r) = invert_pose([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]);
        for value in t {
            close(value, 0.0);
        }
        close(r[0], 0.0);
        close(r[1], 0.0);
        close(r[2], 0.0);
        close(r[3], 1.0);
    }

    #[test]
    fn inverting_pure_translation_negates_it() {
        let (t, r) = invert_pose([1.0, -2.0, 3.0], [0.0, 0.0, 0.0, 1.0]);
        close(t[0], -1.0);
        close(t[1], 2.0);
        close(t[2], -3.0);
        close(r[3], 1.0);
    }

    #[test]
    fn inverting_tcw_recovers_camera_position_in_world() {
        // Camera at world position (2, 0, 0), yawed +90 deg about z (Rwc). Then
        //   Rcw = Rwc^-1 (yaw -90), tcw = -Rcw * twc.
        // invert_pose(Tcw) must recover twc = (2, 0, 0) and Rwc (yaw +90).
        let half = std::f64::consts::FRAC_PI_4; // 45 deg => quaternion half-angle for 90 deg
        // Rcw = yaw -90 deg
        let rcw = [0.0, 0.0, -(half).sin(), (half).cos()];
        // twc = (2,0,0); tcw = -Rcw * twc. Rcw rotates (2,0,0) by -90 about z => (0, -2, 0); negate => (0, 2, 0)
        let tcw = [0.0, 2.0, 0.0];
        let (twc, rwc) = invert_pose(tcw, rcw);
        close(twc[0], 2.0);
        close(twc[1], 0.0);
        close(twc[2], 0.0);
        // Rwc = yaw +90 => quaternion z = sin(45), w = cos(45)
        close(rwc[2], half.sin());
        close(rwc[3], half.cos());
    }

    #[test]
    fn quat_mul_preserves_identity() {
        let identity = [0.0, 0.0, 0.0, 1.0];
        let yaw_90 = yaw_quaternion(std::f64::consts::FRAC_PI_2);

        assert_quat_close(quat_mul(identity, yaw_90), yaw_90);
        assert_quat_close(quat_mul(yaw_90, identity), yaw_90);
    }

    #[test]
    fn quat_mul_composes_known_ninety_degree_rotations() {
        let yaw_90 = yaw_quaternion(std::f64::consts::FRAC_PI_2);
        let yaw_180 = yaw_quaternion(std::f64::consts::PI);

        assert_quat_close(quat_mul(yaw_90, yaw_90), yaw_180);
    }

    #[test]
    fn compose_poses_preserves_identity() {
        let (translation, rotation) = compose_poses(
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 2.0, 3.0],
            yaw_quaternion(std::f64::consts::FRAC_PI_2),
        );

        assert_array_close(translation, [1.0, 2.0, 3.0]);
        assert_quat_close(rotation, yaw_quaternion(std::f64::consts::FRAC_PI_2));
    }

    #[test]
    fn compose_poses_adds_pure_translation() {
        let (translation, rotation) = compose_poses(
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
            [4.0, -5.0, 6.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        assert_array_close(translation, [5.0, -3.0, 9.0]);
        assert_quat_close(rotation, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn compose_poses_rotates_child_translation_before_adding() {
        let yaw_90 = yaw_quaternion(std::f64::consts::FRAC_PI_2);
        let (translation, rotation) =
            compose_poses([1.0, 2.0, 0.0], yaw_90, [2.0, 0.0, 0.0], yaw_90);

        assert_array_close(translation, [1.0, 4.0, 0.0]);
        assert_quat_close(rotation, yaw_quaternion(std::f64::consts::PI));
    }

    #[test]
    fn rotation_matrix_from_identity_quaternion_is_identity() {
        let matrix = rotation_matrix_from_quaternion([0.0, 0.0, 0.0, 1.0]);

        assert_array_close(matrix[0], [1.0, 0.0, 0.0]);
        assert_array_close(matrix[1], [0.0, 1.0, 0.0]);
        assert_array_close(matrix[2], [0.0, 0.0, 1.0]);
    }

    #[test]
    fn rotation_matrix_from_x_axis_half_turn_flips_y_and_z() {
        let matrix = rotation_matrix_from_quaternion([1.0, 0.0, 0.0, 0.0]);

        assert_array_close(matrix[0], [1.0, 0.0, 0.0]);
        assert_array_close(matrix[1], [0.0, -1.0, 0.0]);
        assert_array_close(matrix[2], [0.0, 0.0, -1.0]);
    }

    #[test]
    fn camera_optical_from_link_matches_rep_103_axis_semantics() {
        assert_array_close(
            rotate_vector_by_quaternion(CAMERA_OPTICAL_FROM_LINK, [0.0, 0.0, 1.0]),
            [1.0, 0.0, 0.0],
        );
        assert_array_close(
            rotate_vector_by_quaternion(CAMERA_OPTICAL_FROM_LINK, [1.0, 0.0, 0.0]),
            [0.0, -1.0, 0.0],
        );
        assert_array_close(
            rotate_vector_by_quaternion(CAMERA_OPTICAL_FROM_LINK, [0.0, 1.0, 0.0]),
            [0.0, 0.0, -1.0],
        );
    }

    #[test]
    fn camera_optical_to_base_extrinsic_round_trips_through_orb_camera_pose() {
        let link_translation = [1.0, 2.0, 3.0];
        let link_rotation = yaw_quaternion(std::f64::consts::FRAC_PI_2);
        let camera_optical_to_base =
            camera_optical_to_base_extrinsic(link_translation, link_rotation);

        let (base_translation, base_rotation) = compose_poses(
            [10.0, 20.0, 30.0],
            yaw_quaternion(std::f64::consts::FRAC_PI_2),
            camera_optical_to_base.0,
            camera_optical_to_base.1,
        );

        assert_array_close(base_translation, [7.0, 19.0, 28.0]);
        assert_quat_close(base_rotation, [0.5, 0.5, 0.5, 0.5]);
    }
}
