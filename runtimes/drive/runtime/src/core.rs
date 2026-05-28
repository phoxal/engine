pub struct DifferentialDrive {
    pub wheel_radius_m: f64,
    pub wheel_base_m: f64,
}

impl DifferentialDrive {
    /// Converts a body twist into left and right wheel angular speeds.
    pub fn invert(&self, linear_mps: f64, angular_radps: f64) -> (f64, f64) {
        let half_track = self.wheel_base_m / 2.0;
        let v_left = linear_mps - angular_radps * half_track;
        let v_right = linear_mps + angular_radps * half_track;
        (v_left / self.wheel_radius_m, v_right / self.wheel_radius_m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differential_inversion_straight_forward() {
        let (left, right) = DifferentialDrive {
            wheel_radius_m: 0.10,
            wheel_base_m: 0.40,
        }
        .invert(1.0, 0.0);

        assert_near(left, 10.0);
        assert_near(right, 10.0);
    }

    #[test]
    fn differential_inversion_pure_rotation() {
        let (left, right) = DifferentialDrive {
            wheel_radius_m: 0.10,
            wheel_base_m: 0.40,
        }
        .invert(0.0, 1.0);

        assert_near(left, -2.0);
        assert_near(right, 2.0);
    }

    #[test]
    fn differential_inversion_arc() {
        let (left, right) = DifferentialDrive {
            wheel_radius_m: 0.10,
            wheel_base_m: 0.40,
        }
        .invert(0.5, 0.5);

        assert_near(left, 4.0);
        assert_near(right, 6.0);
    }

    fn assert_near(actual: f64, expected: f64) {
        let error = (actual - expected).abs();
        assert!(
            error <= 1.0e-12,
            "expected {actual} to be within 1e-12 of {expected}, error {error}"
        );
    }
}
