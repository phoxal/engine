const WGS84_SEMI_MAJOR_AXIS_M: f64 = 6_378_137.0;
const WGS84_FLATTENING: f64 = 1.0 / 298.257_223_563;
const WGS84_ECCENTRICITY_SQUARED: f64 = WGS84_FLATTENING * (2.0 - WGS84_FLATTENING);

/// Converts WGS84 geodetic latitude/longitude/altitude to local ENU meters.
///
/// This uses the standard WGS84 ellipsoid conversion to Earth-Centered,
/// Earth-Fixed coordinates, then projects the ECEF delta into the reference
/// datum's local tangent plane. The output is exact for that tangent-plane
/// definition in floating-point arithmetic; it is not a geodesic arc length.
/// For robot-scale local navigation areas the curvature difference is far below
/// typical GNSS noise, while kilometer-scale offsets remain suitable as a local
/// ENU displacement anchored at the first fix.
pub(crate) fn geodetic_to_enu(
    lat_deg: f64,
    lon_deg: f64,
    alt_m: f64,
    ref_lat_deg: f64,
    ref_lon_deg: f64,
    ref_alt_m: f64,
) -> [f64; 3] {
    let [x, y, z] = geodetic_to_ecef(lat_deg, lon_deg, alt_m);
    let [ref_x, ref_y, ref_z] = geodetic_to_ecef(ref_lat_deg, ref_lon_deg, ref_alt_m);
    let dx = x - ref_x;
    let dy = y - ref_y;
    let dz = z - ref_z;

    let ref_lat_rad = ref_lat_deg.to_radians();
    let ref_lon_rad = ref_lon_deg.to_radians();
    let sin_lat = ref_lat_rad.sin();
    let cos_lat = ref_lat_rad.cos();
    let sin_lon = ref_lon_rad.sin();
    let cos_lon = ref_lon_rad.cos();

    [
        -sin_lon * dx + cos_lon * dy,
        -sin_lat * cos_lon * dx - sin_lat * sin_lon * dy + cos_lat * dz,
        cos_lat * cos_lon * dx + cos_lat * sin_lon * dy + sin_lat * dz,
    ]
}

fn geodetic_to_ecef(lat_deg: f64, lon_deg: f64, alt_m: f64) -> [f64; 3] {
    let lat_rad = lat_deg.to_radians();
    let lon_rad = lon_deg.to_radians();
    let sin_lat = lat_rad.sin();
    let cos_lat = lat_rad.cos();
    let sin_lon = lon_rad.sin();
    let cos_lon = lon_rad.cos();
    let prime_vertical_radius_m =
        WGS84_SEMI_MAJOR_AXIS_M / (1.0 - WGS84_ECCENTRICITY_SQUARED * sin_lat * sin_lat).sqrt();

    [
        (prime_vertical_radius_m + alt_m) * cos_lat * cos_lon,
        (prime_vertical_radius_m + alt_m) * cos_lat * sin_lon,
        (prime_vertical_radius_m * (1.0 - WGS84_ECCENTRICITY_SQUARED) + alt_m) * sin_lat,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLOSE: f64 = 1.0e-6;

    #[test]
    fn zero_offset_at_reference() {
        let enu = geodetic_to_enu(52.225_611, 6.883_363_5, 12.0, 52.225_611, 6.883_363_5, 12.0);

        assert_close(enu[0], 0.0, CLOSE);
        assert_close(enu[1], 0.0, CLOSE);
        assert_close(enu[2], 0.0, CLOSE);
    }

    #[test]
    fn one_degree_latitude_is_about_111_km_north() {
        let enu = geodetic_to_enu(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        assert_close(enu[0], 0.0, CLOSE);
        assert_close(enu[1], 110_568.0, 1_000.0);
    }

    #[test]
    fn east_offset_scales_with_latitude() {
        let lat_deg: f64 = 45.0;
        let enu = geodetic_to_enu(lat_deg, 1.0, 0.0, lat_deg, 0.0, 0.0);
        let equatorial_degree_m = WGS84_SEMI_MAJOR_AXIS_M * 1.0_f64.to_radians();
        let expected_east_m = equatorial_degree_m * lat_deg.to_radians().cos();

        assert_close(enu[0], expected_east_m, 1_000.0);
        assert!(enu[1].abs() < 1_000.0);
    }

    #[test]
    fn altitude_maps_to_up() {
        let enu = geodetic_to_enu(52.225_611, 6.883_363_5, 27.0, 52.225_611, 6.883_363_5, 12.0);

        assert_close(enu[0], 0.0, CLOSE);
        assert_close(enu[1], 0.0, CLOSE);
        assert_close(enu[2], 15.0, CLOSE);
    }

    #[test]
    fn round_trips_small_local_offset() {
        let ref_lat_deg: f64 = 52.225_611;
        let ref_lon_deg: f64 = 6.883_363_5;
        let ref_alt_m = 12.0;
        let target_east_m = 5.0;
        let target_north_m = -3.0;
        let target_up_m = 2.0;
        let lat_rad = ref_lat_deg.to_radians();
        let sin_lat = lat_rad.sin();
        let prime_vertical_radius_m =
            WGS84_SEMI_MAJOR_AXIS_M / (1.0 - WGS84_ECCENTRICITY_SQUARED * sin_lat * sin_lat).sqrt();
        let meridian_radius_m = WGS84_SEMI_MAJOR_AXIS_M * (1.0 - WGS84_ECCENTRICITY_SQUARED)
            / (1.0 - WGS84_ECCENTRICITY_SQUARED * sin_lat * sin_lat).powf(1.5);
        let target_lat_deg =
            ref_lat_deg + (target_north_m / (meridian_radius_m + ref_alt_m)).to_degrees();
        let target_lon_deg = ref_lon_deg
            + (target_east_m / ((prime_vertical_radius_m + ref_alt_m) * lat_rad.cos()))
                .to_degrees();
        let target_alt_m = ref_alt_m + target_up_m;

        let enu = geodetic_to_enu(
            target_lat_deg,
            target_lon_deg,
            target_alt_m,
            ref_lat_deg,
            ref_lon_deg,
            ref_alt_m,
        );

        assert_close(enu[0], target_east_m, 0.001);
        assert_close(enu[1], target_north_m, 0.001);
        assert_close(enu[2], target_up_m, 0.001);
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {actual} to be within {tolerance} of {expected}"
        );
    }
}
