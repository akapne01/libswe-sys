use crate::raw::{self, swe_houses_ex2};
use crate::swerust::ffi_util::{buf_to_string, new_err_buf};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

/*
 * 14. House cusp calculation
 */

pub fn house_name(hsys: char) -> String {
    unsafe {
        CString::from(CStr::from_ptr(raw::swe_house_name(hsys as c_int)))
            .to_str()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HousesResult {
    // cusps: [f64; 37], // Limtation to 32 ->
    // /* array for 13 (or 37 for system G) doubles */
    pub cusps: Vec<f64>,
    pub ascmc: [f64; 10],
    pub result: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HouseCalculationResult {
    pub cusps: [f64; 12],       // House cusps 1-12
    pub ascmc: [f64; 8],        // Asc, MC, etc.
    pub cusp_speeds: [f64; 12], // House cusp motion
    pub ascmc_speeds: [f64; 8], // Angular point motion
}

pub fn houses(
    tjd_ut: f64,
    geolat: f64,
    geolong: f64,
    hsys: char,
) -> HousesResult {
    let mut cusps = [0.0; 37];
    let mut ascmc = [0.0; 10];
    let result: i32 = unsafe {
        let p_cuspsw = cusps.as_mut_ptr();
        let p_ascmc = ascmc.as_mut_ptr();
        raw::swe_houses_ex(
            tjd_ut,
            0, // 64 | (64 * 1024),
            geolat,
            geolong,
            hsys as c_int,
            p_cuspsw,
            p_ascmc,
        )
    };
    HousesResult {
        cusps: cusps.to_vec(),
        ascmc,
        result,
    }
}

pub fn houses_with_flag(
    tjd_ut: f64,
    flag: i32,
    geolat: f64,
    geolong: f64,
    hsys: char,
) -> HousesResult {
    let mut cusps = [0.0; 37];
    let mut ascmc = [0.0; 10];
    let result: i32 = unsafe {
        let p_cuspsw = cusps.as_mut_ptr();
        let p_ascmc = ascmc.as_mut_ptr();
        raw::swe_houses_ex(
            tjd_ut,
            flag,
            geolat,
            geolong,
            hsys as c_int,
            p_cuspsw,
            p_ascmc,
        )
    };
    HousesResult {
        cusps: cusps.to_vec(),
        ascmc,
        result,
    }
}

/// swephexp.h: `SE_NASCMC`. The C library treats `ascmc` / `ascmc_speed` as
/// `double[10]`. Only the first 8 entries are meaningful, but all 10 may be written.
const SE_NASCMC: usize = 10;

/// swe_houses_ex2 needs 13 cusp slots (index 0 is unused). 37 is only needed for
/// Gauquelin ('G'), which is rejected below, but the extra room is harmless.
const SE_NCUSPS_BUF: usize = 37;

pub fn calculate_houses_extended_with_speeds(
    jd_ut: f64,
    iflag: i32,
    lat: f64,
    lon: f64,
    hsys: char,
) -> Result<HouseCalculationResult, String> {
    if hsys == 'G' {
        return Err("GAUQUELIN_SECTORS is not implemented.".to_string());
    }

    let mut cusps = [0.0_f64; SE_NCUSPS_BUF];
    let mut ascmc = [0.0_f64; SE_NASCMC];
    let mut cusp_speeds = [0.0_f64; SE_NCUSPS_BUF];
    let mut ascmc_speeds = [0.0_f64; SE_NASCMC];
    let mut serr = new_err_buf(); // [c_char; 256]

    let res = unsafe {
        swe_houses_ex2(
            jd_ut,
            iflag,
            lat,
            lon,
            hsys as c_int,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
            cusp_speeds.as_mut_ptr(),
            ascmc_speeds.as_mut_ptr(),
            serr.as_mut_ptr(),
        )
    };

    if res == -1 {
        let err_msg = buf_to_string(&serr);
        return Err(if err_msg.is_empty() {
            "Unknown error occurred while calculating houses".to_string()
        } else {
            err_msg
        });
    }

    tracing::debug!("All House cusps: {:?}", &cusps[..13]);
    tracing::debug!("All House cusp speeds: {:?}", &cusp_speeds[..13]);

    // Index 0 of cusps / cusp_speeds is unused by Swiss Ephemeris; houses are 1..=12.
    let house_cusps: [f64; 12] = cusps[1..13]
        .try_into()
        .map_err(|e| format!("Failed to extract house cusps: {e}"))?;
    let house_speeds: [f64; 12] = cusp_speeds[1..13]
        .try_into()
        .map_err(|e| format!("Failed to extract cusp speeds: {e}"))?;

    // Keep the public struct at 8 entries; drop the 2 reserved ones.
    let ascmc8: [f64; 8] = ascmc[..8]
        .try_into()
        .map_err(|e| format!("Failed to extract ascmc: {e}"))?;
    let ascmc_speeds8: [f64; 8] = ascmc_speeds[..8]
        .try_into()
        .map_err(|e| format!("Failed to extract ascmc speeds: {e}"))?;

    Ok(HouseCalculationResult {
        cusps: house_cusps,
        ascmc: ascmc8,
        cusp_speeds: house_speeds,
        ascmc_speeds: ascmc_speeds8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{Ayanamsha, CalculationFlags, HouseSystems},
        ensure_ephemeris_initialized,
        sweconst::Calendar,
        swerust::{
            handler_swe03::set_sidereal_mode,
            handler_swe08::{utc_time_zone, utc_to_jd},
        },
    };

    #[test]
    pub fn test_house_calculation() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;
        let flag = CalculationFlags::SPEED_PRECISION;

        let expected_result = HousesResult {
            cusps: [
                0.0,
                48.62763222672374,
                75.71944896860263,
                96.49287030684448,
                117.11096726546964,
                142.2785541384614,
                178.74827673009713,
                228.62763222672373,
                255.71944896860265,
                276.49287030684445,
                297.11096726546964,
                322.2785541384614,
                358.74827673009713,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
            ]
            .to_vec(),
            ascmc: [
                48.62763222672374,
                297.11096726546964,
                299.1619604398136,
                201.66776865408326,
                31.307700535881203,
                22.55507848219912,
                52.35280295047079,
                202.55507848219912,
                0.0,
                0.0,
            ],
            result: 0,
        };

        let actual_result =
            houses_with_flag(date, flag, lat, lng, house_system);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    pub fn test_house_calculation_extended_with_speeds() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;

        let expected_result = HouseCalculationResult {
            cusps: [
                48.62763222672374,
                75.71944896860263,
                96.49287030684448,
                117.11096726546964,
                142.2785541384614,
                178.74827673009713,
                228.62763222672373,
                255.71944896860265,
                276.49287030684445,
                297.11096726546964,
                322.2785541384614,
                358.74827673009713,
            ],
            ascmc: [
                48.62763222672374,
                297.11096726546964,
                299.1619604398136,
                201.66776865408326,
                31.307700535881203,
                22.55507848219912,
                52.35280295047079,
                202.55507848219912,
            ],
            cusp_speeds: [
                507.3817187152055,
                362.56618362266704,
                326.3715539865318,
                344.12778754362796,
                416.59494083438483,
                539.1280261881008,
                507.3817187152055,
                362.56618362266704,
                326.3715539865318,
                344.12778754362796,
                416.59494083438483,
                539.1280261881008,
            ],
            ascmc_speeds: [
                507.3817187152055,
                344.12778754362796,
                360.98564733498665,
                267.1306504408959,
                376.64136009832293,
                277.88707643085064,
                520.5003314381933,
                277.88707643085064,
            ],
        };

        let actual_result = calculate_houses_extended_with_speeds(
            date,
            0,
            lat,
            lng,
            house_system,
        );

        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_result);
    }

    #[test]
    pub fn test_house_calculation_extended_with_speeds_sidereal_calculations() {
        let _ = ensure_ephemeris_initialized();
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;
        set_sidereal_mode(Ayanamsha::GALACTIC_CENTER_MULA_WILHELM);
        let flag = CalculationFlags::SIDEREAL_POSITIONS;

        let expected_result = HouseCalculationResult {
            cusps: [
                28.225351258990685,
                55.31716800086958,
                76.09058933911143,
                96.70868629773659,
                121.87627317072835,
                158.34599576236408,
                208.22535125899068,
                235.3171680008696,
                256.0905893391114,
                276.70868629773656,
                301.8762731707283,
                338.34599576236405,
            ],
            ascmc: [
                28.225351258990685,
                276.70868629773656,
                299.1619604398136,
                181.2654876863502,
                10.905419568148147,
                2.152797514466066,
                31.950521982737733,
                182.15279751446607,
            ],
            cusp_speeds: [
                507.3817187152055,
                362.56618362266704,
                326.3715539865318,
                344.12778754362796,
                416.59494083438483,
                539.1280261881008,
                507.3817187152055,
                362.56618362266704,
                326.3715539865318,
                344.12778754362796,
                416.59494083438483,
                539.1280261881008,
            ],
            ascmc_speeds: [
                507.3817187152055,
                344.12778754362796,
                360.98564733498665,
                267.1306504408959,
                376.64136009832293,
                277.88707643085064,
                520.5003314381933,
                277.88707643085064,
            ],
        };

        let actual_result = calculate_houses_extended_with_speeds(
            date,
            flag,
            lat,
            lng,
            house_system,
        );

        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_result);
    }

    #[test]
    fn test_calculate_houses_extended_with_speeds_error_case_when_calling_not_implemented_house_system(
    ) {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let iflag = 0;
        let hsys = 'G'; // Invalid house system

        let result =
            calculate_houses_extended_with_speeds(date, iflag, lat, lng, hsys);

        assert!(result.is_err(), "Expected an error but got Ok");
        if let Err(err_msg) = result {
            println!("Error message: {}", err_msg);
            assert!(!err_msg.is_empty(), "Error message should not be empty");
        }
    }

    #[test]
    fn test_extended_houses_invariants() {
        let _ = ensure_ephemeris_initialized();
        let jd = 2449298.885420433;
        let r =
            calculate_houses_extended_with_speeds(jd, 0, 51.5074, -0.1278, 'P')
                .unwrap();

        // Cusp 1 is the ascendant for Placidus
        assert!((r.cusps[0] - r.ascmc[0]).abs() < 1e-9);
        // Cusp 7 is opposite cusp 1
        let opp = (r.cusps[0] + 180.0).rem_euclid(360.0);
        assert!((r.cusps[6] - opp).abs() < 1e-9);
        // Speeds follow the same rules
        assert!((r.cusp_speeds[0] - r.ascmc_speeds[0]).abs() < 1e-9);
        assert!((r.cusp_speeds[0] - r.cusp_speeds[6]).abs() < 1e-9);
    }

    #[test]
    fn swe_houses_ex2_writes_ten_ascmc_entries() {
        use crate::swerust::ffi_util::new_err_buf;

        let _ = ensure_ephemeris_initialized();

        // Deliberately oversized buffers, filled with NaN so we can see what C wrote.
        let mut cusps = [f64::NAN; 37];
        let mut ascmc = [f64::NAN; 16];
        let mut cusp_speeds = [f64::NAN; 37];
        let mut ascmc_speeds = [f64::NAN; 16];
        let mut serr = new_err_buf();

        let res = unsafe {
            swe_houses_ex2(
                2449298.885420433,
                0,
                51.5074,
                -0.1278,
                'P' as i32,
                cusps.as_mut_ptr(),
                ascmc.as_mut_ptr(),
                cusp_speeds.as_mut_ptr(),
                ascmc_speeds.as_mut_ptr(),
                serr.as_mut_ptr(),
            )
        };
        assert_ne!(res, -1, "swe_houses_ex2 failed");

        let written = ascmc.iter().take_while(|v| !v.is_nan()).count();
        let written_speeds =
            ascmc_speeds.iter().take_while(|v| !v.is_nan()).count();
        println!("ascmc entries written: {written}, ascmc_speeds entries written: {written_speeds}");

        assert_eq!(written, 10); // confirms indices 8 and 9 are written
        assert_eq!(written_speeds, 10);
    }

    pub fn get_test_date_time() -> f64 {
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
}
