use std::ffi::{CStr, CString};

use crate::{raw::swe_nod_aps_ut, sweconst::Bodies};

/// Mean Nodes are available only for planets Moon, Mercury to Neptune.
/// Pluto and Asteroids always return True Nodes, even if Means nodes are selected.
pub struct CalculationMethodsNodesApsides;

impl CalculationMethodsNodesApsides {
    pub const MEAN_NODES_AND_APSIDES: i32 = 1; // MO, ME-> NE mean nodes, for PL & asteroids true nodes
    pub const TRUE_NODES_AND_APSIDES: i32 = 2; // Osculating (True) Nodes for all bodies
    pub const TRUE_NODES_AND_APSIDES_BARYCENTRIC_AFTER_JU: i32 = 4; // Osculating (True) Nodes for all bodies. With planets beyond Jupiter, the nodes and apsides are calculated from barycentric positions and speed.
    pub const MEAN_NODES_AND_APSIDES_BARYCENTRIC_AFTER_JU: i32 = 5; // Mean nodes only up to NE. With planets beyond Jupiter, the nodes and apsides are calculated from barycentric positions and speed.
    pub const SECOND_FOCAL_POINT_ORBITAL_ELLIPSE: i32 = 256; // The second focal point of the orbital ellipse is computed and returned in the array of the aphelion. This bit can be combined with any other bit.
}

#[derive(Debug, Clone, PartialEq)]
pub struct CelestialPosition {
    pub longitude: f64,            // Ecliptic Longitude (degrees)
    pub latitude: f64,             // Ecliptic Latitude (degrees)
    pub distance_from_sun_au: f64, // Distance from Sun (AU)
    pub longitude_speed: f64,      // Change in longitude per day (deg/day)
    pub latitude_speed: f64,       // Change in latitude per day (deg/day)
    pub radial_speed: f64,         // Change in distance per day (AU/day)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApsidesAndNodesResult {
    pub body: Bodies,
    pub north_node: CelestialPosition, // Where planet's orbit crosses the ecliptic going north
    pub south_node: CelestialPosition, // Where planet's orbit crosses the ecliptic going north
    pub perihelion: CelestialPosition, // Closest point to the Sun
    pub aphelion: CelestialPosition,   // Farthest point from the Sun
}

/// Calculates Planetary Nodes and Apsides: perihelia, aphelia, second focal points of the orbital ellipses.
/// Breakdown of All 6 Values
// Each array (xnasc, xndsc, xperi, xaphe) contains:
// Index Meaning	                Explanation
// 0	 Longitude (λ)	            Ecliptic longitude of the object (degrees)
// 1	 Latitude (β)	            Ecliptic latitude of the object (degrees)
// 2	 Distance (r)	            Distance from the Sun (AU)
// 3	 Longitude Speed (dλ/dt)	Change in longitude per day (deg/day)
// 4	 Latitude Speed (dβ/dt) 	Change in latitude per day (deg/day)
// 5	 Radial Speed (dr/dt)	    Change in distance per day (AU/day)
pub fn get_planetary_apsides_and_nodes(
    body: Bodies,
    jd: f64,
    iflag: i32,
    method: i32,
) -> Result<ApsidesAndNodesResult, String> {
    let mut xnasc = [0.0; 6]; // North Node
    let mut xndsc = [0.0; 6]; // South Node
    let mut xperi = [0.0; 6]; // Perihelion
    let mut xaphe = [0.0; 6]; // Aphelion
    let mut serr = [0; 255]; // Error buffer
    let p_serr = serr.as_mut_ptr();

    unsafe {
        swe_nod_aps_ut(
            jd,
            body as i32,
            iflag,
            method,
            xnasc.as_mut_ptr(),
            xndsc.as_mut_ptr(),
            xperi.as_mut_ptr(),
            xaphe.as_mut_ptr(),
            p_serr,
        );

        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        if !s_serr.is_empty() {
            return Err(s_serr);
        }
    }

    Ok(ApsidesAndNodesResult {
        north_node: CelestialPosition {
            longitude: xnasc[0],
            latitude: xnasc[1],
            distance_from_sun_au: xnasc[2],
            longitude_speed: xnasc[3],
            latitude_speed: xnasc[4],
            radial_speed: xnasc[5],
        },
        south_node: CelestialPosition {
            longitude: xndsc[0],
            latitude: xndsc[1],
            distance_from_sun_au: xndsc[2],
            longitude_speed: xndsc[3],
            latitude_speed: xndsc[4],
            radial_speed: xndsc[5],
        },
        perihelion: CelestialPosition {
            longitude: xperi[0],
            latitude: xperi[1],
            distance_from_sun_au: xperi[2],
            longitude_speed: xperi[3],
            latitude_speed: xperi[4],
            radial_speed: xperi[5],
        },
        aphelion: CelestialPosition {
            longitude: xaphe[0],
            latitude: xaphe[1],
            distance_from_sun_au: xaphe[2],
            longitude_speed: xaphe[3],
            latitude_speed: xaphe[4],
            radial_speed: xaphe[5],
        },
        body,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::CalculationFlags,
        ensure_ephemeris_initialized,
        sweconst::{Bodies, Calendar},
        swerust::handler_swe08::{utc_time_zone, utc_to_jd},
    };

    #[test]
    pub fn test_get_planetary_apsides_and_nodes_when_true_nodes_no_speed_precision(
    ) {
        let _ = ensure_ephemeris_initialized();
        let jd = get_test_julian_time();
        let expected_result = ApsidesAndNodesResult {
            body: Bodies::Pluto,
            north_node: CelestialPosition {
                longitude: 111.7575745157746,
                latitude: 3.3106999730182626e-6,
                distance_from_sun_au: 40.43162210909668,
                longitude_speed: 0.0,
                latitude_speed: 0.0,
                radial_speed: 0.0,
            },
            south_node: CelestialPosition {
                longitude: 289.38325356248436,
                latitude: 3.8877641639265405e-6,
                distance_from_sun_au: 34.30367571339855,
                longitude_speed: 0.0,
                latitude_speed: 0.0,
                radial_speed: 0.0,
            },
            perihelion: CelestialPosition {
                longitude: 225.71759376427664,
                latitude: 15.20571407932947,
                distance_from_sun_au: 30.601998795696517,
                longitude_speed: 0.0,
                latitude_speed: 0.0,
                radial_speed: 0.0,
            },
            aphelion: CelestialPosition {
                longitude: 44.94228129867987,
                latitude: -15.998542070518234,
                distance_from_sun_au: 48.1803777926731,
                longitude_speed: 0.0,
                latitude_speed: 0.0,
                radial_speed: 0.0,
            },
        };

        let iflag = CalculationFlags::SWISS_EPHEMERIS;

        let actual_result = get_planetary_apsides_and_nodes(
            Bodies::Pluto,
            jd,
            iflag,
            CalculationMethodsNodesApsides::TRUE_NODES_AND_APSIDES,
        );

        assert!(actual_result.is_ok());
        assert_apsides_and_nodes_approx(
            &actual_result.unwrap(),
            &expected_result,
            1e-6,
        );
    }

    #[test]
    pub fn test_get_planetary_apsides_and_nodes_when_true_nodes_when_speed_precision_specified(
    ) {
        let _ = ensure_ephemeris_initialized();
        let jd = get_test_julian_time();
        let expected_result = ApsidesAndNodesResult {
            body: Bodies::Pluto,
            north_node: CelestialPosition {
                longitude: 111.75757451577459,
                latitude: 3.310699973017955e-6,
                distance_from_sun_au: 40.431622109096686,
                longitude_speed: -0.015227867985212955,
                latitude_speed: -6.758638745711786e-7,
                radial_speed: -0.014259561246940841,
            },
            south_node: CelestialPosition {
                longitude: 289.38325151126105,
                latitude: 3.9308906632423146e-6,
                distance_from_sun_au: 34.3036742656753,
                longitude_speed: 0.01908734174972315,
                latitude_speed: -8.071179900318823e-7,
                radial_speed: 0.013159190151014877,
            },
            perihelion: CelestialPosition {
                longitude: 225.71759074318769,
                latitude: 15.20571399138036,
                distance_from_sun_au: 30.601999170952066,
                longitude_speed: 0.031018794220375893,
                latitude_speed: 0.0023945948754551414,
                radial_speed: -0.004413765321743456,
            },
            aphelion: CelestialPosition {
                longitude: 44.94228301258608,
                latitude: -15.998542166293118,
                distance_from_sun_au: 48.180377440557606,
                longitude_speed: -0.022574199832243208,
                latitude_speed: 0.0012260583942533664,
                radial_speed: 0.0040172833176065625,
            },
        };

        let iflag = CalculationFlags::SWISS_EPHEMERIS
            + CalculationFlags::SPEED_PRECISION;

        let actual_result = get_planetary_apsides_and_nodes(
            Bodies::Pluto,
            jd,
            iflag,
            CalculationMethodsNodesApsides::TRUE_NODES_AND_APSIDES,
        );
        assert!(actual_result.is_ok());
        assert_apsides_and_nodes_approx(
            &actual_result.unwrap(),
            &expected_result,
            1e-6,
        );
    }

    pub fn get_test_julian_time() -> f64 {
        let utc_time_zone = utc_time_zone(2024, 11, 21, 16, 10, 0.0, 2.0);
        let jd = utc_to_jd(
            utc_time_zone.year[0],
            utc_time_zone.month[0],
            utc_time_zone.day[0],
            utc_time_zone.hour[0],
            utc_time_zone.min[0],
            utc_time_zone.sec[0],
            Calendar::Gregorian,
        );
        jd.julian_day_ut
    }

    fn assert_celestial_position_approx(
        a: &CelestialPosition,
        b: &CelestialPosition,
        epsilon: f64,
    ) {
        assert!(
            (a.longitude - b.longitude).abs() <= epsilon,
            "longitude differs: {} != {}",
            a.longitude,
            b.longitude
        );
        assert!(
            (a.latitude - b.latitude).abs() <= epsilon,
            "latitude differs: {} != {}",
            a.latitude,
            b.latitude
        );
        assert!(
            (a.distance_from_sun_au - b.distance_from_sun_au).abs() <= epsilon,
            "distance differs: {} != {}",
            a.distance_from_sun_au,
            b.distance_from_sun_au
        );
        assert!(
            (a.longitude_speed - b.longitude_speed).abs() <= epsilon,
            "longitude_speed differs: {} != {}",
            a.longitude_speed,
            b.longitude_speed
        );
        assert!(
            (a.latitude_speed - b.latitude_speed).abs() <= epsilon,
            "latitude_speed differs: {} != {}",
            a.latitude_speed,
            b.latitude_speed
        );
        assert!(
            (a.radial_speed - b.radial_speed).abs() <= epsilon,
            "radial_speed differs: {} != {}",
            a.radial_speed,
            b.radial_speed
        );
    }

    fn assert_apsides_and_nodes_approx(
        a: &ApsidesAndNodesResult,
        b: &ApsidesAndNodesResult,
        epsilon: f64,
    ) {
        assert_eq!(a.body, b.body);
        assert_celestial_position_approx(&a.north_node, &b.north_node, epsilon);
        assert_celestial_position_approx(&a.south_node, &b.south_node, epsilon);
        assert_celestial_position_approx(&a.perihelion, &b.perihelion, epsilon);
        assert_celestial_position_approx(&a.aphelion, &b.aphelion, epsilon);
    }
}
