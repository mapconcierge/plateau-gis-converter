//! Web Mercator projection utilities.

use std::f64::consts::FRAC_PI_2;

/// Converts geographic coordinate (lng, lat) to Web Mercator coordinate (mx, my).
///
/// The range of (mx, my) is [0.0, 0.0]-[1.0, 1.0] (same as Mapbox/MapLibre API, etc.)
pub fn lnglat_to_web_mercator(lng: f64, lat: f64) -> (f64, f64) {
    let mx = (lng + 180.0) / 360.0;
    let my = ((90.0 + lat).to_radians() / 2.0).tan().ln().to_degrees();
    let my = (-my + 180.0) / 360.0;
    (mx, my)
}

/// Converts Web Mercator coordinate (mx, my) to geographic coordinate (lng, lat).
///
/// The range of (mx, my) is [0.0, 0.0]-[1.0, 1.0] (same as Mapbox/MapLibre API, etc.)
pub fn web_mercator_to_lnglat(mx: f64, my: f64) -> (f64, f64) {
    let lng = mx * 360.0 - 180.0;
    let lat = my * 360.0 - 180.0;
    let lat = -(2.0 * (lat.to_radians().exp()).atan() - FRAC_PI_2).to_degrees();
    (lng, lat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        {
            let (lng, lat) = (136.08, 37.39);
            let (mx, my) = lnglat_to_web_mercator(lng, lat);
            let (lng2, lat2) = web_mercator_to_lnglat(mx, my);
            assert!((lng - lng2).abs() < 1e-13);
            assert!((lat - lat2).abs() < 1e-13);
        }
        {
            let (lng, lat) = (0.3, 0.2);
            let (mx, my) = lnglat_to_web_mercator(lng, lat);
            let (lng2, lat2) = web_mercator_to_lnglat(mx, my);
            assert!((lng - lng2).abs() < 1e-13);
            assert!((lat - lat2).abs() < 1e-13);
        }
    }

    #[test]
    fn null_island() {
        // https://en.wikipedia.org/wiki/Null_Island
        // (lng: 0, lat: 0) -> (mx: 0.5, my: 0.5)
        let (lng, lat) = (0., 0.);
        let (mx, my) = lnglat_to_web_mercator(lng, lat);
        assert!((mx - 0.5).abs() < 1e-10);
        assert!((my - 0.5).abs() < 1e-10);
    }
}
