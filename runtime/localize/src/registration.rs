use anyhow::{Context as _, Result, bail};

use crate::settings::CameraIntrinsics;

/// Maps color pixels to source depth samples for co-located pinhole cameras.
#[derive(Debug, Clone)]
pub(crate) struct DepthRegistration {
    color_width: u32,
    color_height: u32,
    depth_width: u32,
    depth_height: u32,
    /// Row-major over the color grid: source depth sample index, or None when the
    /// color ray falls outside the depth image.
    lut: Vec<Option<usize>>,
}

impl DepthRegistration {
    pub(crate) fn new(color: &CameraIntrinsics, depth: &CameraIntrinsics) -> Self {
        let mut lut = Vec::new();
        for v_c in 0..color.height {
            for u_c in 0..color.width {
                let x = (f64::from(u_c) + 0.5 - color.cx) / color.fx;
                let y = (f64::from(v_c) + 0.5 - color.cy) / color.fy;
                let u_d = depth.fx * x + depth.cx;
                let v_d = depth.fy * y + depth.cy;
                lut.push(depth_index_from_projection(
                    u_d,
                    v_d,
                    depth.width,
                    depth.height,
                ));
            }
        }

        Self {
            color_width: color.width,
            color_height: color.height,
            depth_width: depth.width,
            depth_height: depth.height,
            lut,
        }
    }

    /// Resample a source depth buffer into the color grid. Out-of-range color
    /// pixels become 0, which ORB-SLAM3 treats as "no depth".
    pub(crate) fn register(&self, source_depth_mm: &[u16]) -> Result<Vec<u16>> {
        let expected_len = pixel_count(self.depth_width, self.depth_height)
            .context("depth registration source grid size overflow")?;
        if source_depth_mm.len() != expected_len {
            bail!(
                "depth frame has {} samples, expected {} for configured {}x{} depth grid",
                source_depth_mm.len(),
                expected_len,
                self.depth_width,
                self.depth_height
            );
        }

        let registered = self
            .lut
            .iter()
            .map(|source_index| match source_index {
                Some(index) => source_depth_mm[*index],
                None => 0,
            })
            .collect();
        Ok(registered)
    }

    pub(crate) fn color_pixel_count(&self) -> usize {
        self.lut.len()
    }

    pub(crate) fn color_dimensions(&self) -> (u32, u32) {
        (self.color_width, self.color_height)
    }
}

fn depth_index_from_projection(
    projected_u: f64,
    projected_v: f64,
    width: u32,
    height: u32,
) -> Option<usize> {
    let sample_u = (projected_u - 0.5).round();
    let sample_v = (projected_v - 0.5).round();
    if !sample_u.is_finite()
        || !sample_v.is_finite()
        || sample_u < 0.0
        || sample_v < 0.0
        || sample_u >= f64::from(width)
        || sample_v >= f64::from(height)
    {
        return None;
    }

    let u = sample_u as u32;
    let v = sample_v as u32;
    let index = u64::from(v)
        .checked_mul(u64::from(width))?
        .checked_add(u64::from(u))?;
    usize::try_from(index).ok()
}

fn pixel_count(width: u32, height: u32) -> Option<usize> {
    let count = u64::from(width).checked_mul(u64::from(height))?;
    usize::try_from(count).ok()
}

#[cfg(test)]
mod tests {
    use super::DepthRegistration;
    use crate::settings::CameraIntrinsics;

    #[test]
    fn identity_when_intrinsics_match() {
        let color = must(
            CameraIntrinsics::from_horizontal_fov(640, 480, 1.204277),
            "valid color intrinsics",
        );
        let depth = color.clone();
        let registration = DepthRegistration::new(&color, &depth);
        let source = sequential_depth(640, 480);

        assert_eq!(registration.color_pixel_count(), 640 * 480);
        assert_eq!(must(registration.register(&source), "registered"), source);
    }

    #[test]
    fn registers_oak_d_lite_depth_to_rgb_grid() {
        let color = must(
            CameraIntrinsics::from_horizontal_fov(640, 480, 1.204277),
            "valid oak-d-lite RGB intrinsics",
        );
        let depth = must(
            CameraIntrinsics::from_horizontal_fov(640, 400, 1.272271),
            "valid oak-d-lite depth intrinsics",
        );
        let registration = DepthRegistration::new(&color, &depth);
        let registered = must(
            registration.register(&vec![1500_u16; 640 * 400]),
            "oak-d-lite depth registers into RGB grid",
        );

        assert_eq!(registered.len(), 640 * 480);
        assert_eq!(registered[240 * 640 + 320], 1500);
    }

    #[test]
    fn register_rejects_wrong_source_length() {
        let color = must(
            CameraIntrinsics::from_horizontal_fov(640, 480, 1.2),
            "valid color intrinsics",
        );
        let depth = must(
            CameraIntrinsics::from_horizontal_fov(640, 400, 1.2),
            "valid depth intrinsics",
        );
        let registration = DepthRegistration::new(&color, &depth);

        assert!(
            registration
                .register(&vec![1500_u16; 640 * 400 - 1])
                .is_err()
        );
    }

    #[test]
    fn out_of_range_color_pixels_become_zero() {
        let color = must(
            CameraIntrinsics::from_horizontal_fov(5, 3, 2.4),
            "valid color intrinsics",
        );
        let depth = must(
            CameraIntrinsics::from_horizontal_fov(5, 3, 0.7),
            "valid depth intrinsics",
        );
        let registration = DepthRegistration::new(&color, &depth);
        let registered = must(
            registration.register(&[42_u16; 5 * 3]),
            "registration succeeds",
        );

        assert_eq!(registered[0], 0);
        assert_eq!(registered[7], 42);
        assert_eq!(registered[14], 0);
    }

    fn sequential_depth(width: usize, height: usize) -> Vec<u16> {
        (0..width * height)
            .map(|index| {
                must(
                    u16::try_from(index % usize::from(u16::MAX)),
                    "bounded sample",
                )
            })
            .collect()
    }

    fn must<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("{context}: {error}"),
        }
    }
}
