use crate::raw::{ self, swe_houses_ex2 };
use std::convert::TryInto;
// use crate::sweconst::HouseSystem;
use std::ffi::{ c_char, CStr, CString };
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
    pub cusps: [f64; 12], // House cusps 1-12
    pub ascmc: [f64; 8], // Asc, MC, etc.
    pub cusp_speeds: [f64; 12], // House cusp motion
    pub ascmc_speeds: [f64; 8], // Angular point motion
}

pub fn houses(tjd_ut: f64, geolat: f64, geolong: f64, hsys: char) -> HousesResult {
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
            p_ascmc
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
    hsys: char
) -> HousesResult {
    let mut cusps = [0.0; 37];
    let mut ascmc = [0.0; 10];
    let result: i32 = unsafe {
        let p_cuspsw = cusps.as_mut_ptr();
        let p_ascmc = ascmc.as_mut_ptr();
        raw::swe_houses_ex(tjd_ut, flag, geolat, geolong, hsys as c_int, p_cuspsw, p_ascmc)
    };
    HousesResult {
        cusps: cusps.to_vec(),
        ascmc,
        result,
    }
}

pub fn calculate_houses_extended_with_speeds(
    jd_ut: f64,
    iflag: i32,
    lat: f64,
    lon: f64,
    hsys: char
) -> Result<HouseCalculationResult, String> {
    if hsys == 'G' {
        return Err("GAUQUELIN_SECTORS is not implemented.".to_string());
    }
    let mut cusps = [0.0; 13]; // House cusps
    let mut ascmc = [0.0; 8]; // Angular points (Asc, MC, etc.)
    let mut cusp_speeds = [0.0; 13]; // Motion of house cusps
    let mut ascmc_speeds = [0.0; 8]; // Motion of angular points
    let hsys_int = hsys as i32;

    let mut serr = vec![0u8; 256]; // Error buffer

    let res = unsafe {
        swe_houses_ex2(
            jd_ut,
            iflag,
            lat,
            lon,
            hsys_int,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
            cusp_speeds.as_mut_ptr(),
            ascmc_speeds.as_mut_ptr(),
            serr.as_mut_ptr() as *mut c_char
        )
    };

    if res == -1 {
        let err_msg = unsafe {
            CStr::from_ptr(serr.as_ptr() as *const c_char)
                .to_string_lossy()
                .trim_end_matches('\0')
                .to_string()
        };

        if err_msg.is_empty() {
            return Err("Unknown error occurred while calculating houses".to_string());
        }
        return Err(err_msg);
    }

    let house_cusps: [f64; 12] = cusps[1..].try_into().expect("Failed to extract house cusps");
    let house_speeds: [f64; 12] = cusp_speeds[1..]
        .try_into()
        .expect("Failed to extract cusp speeds");

    Ok(HouseCalculationResult {
        cusps: house_cusps,
        ascmc,
        cusp_speeds: house_speeds,
        ascmc_speeds,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{ Ayanamshas, CalculationFlags, HouseSystems, EPHEMERIS_PATH },
        sweconst::Calendar,
        swerust::{
            handler_swe02::set_ephe_path,
            handler_swe03::set_sidereal_mode,
            handler_swe08::{ utc_time_zone, utc_to_jd },
        },
    };
    use super::*;

    #[test]
    pub fn test_house_calculation() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;
        set_ephe_path(EPHEMERIS_PATH);
        let flag = CalculationFlags::SPEED_PRECISION;

        let expected_result = HousesResult {
            cusps: [
                0.0, 48.62763222672374, 75.71944896860263, 96.49287030684448, 117.11096726546964, 142.2785541384614,
                178.74827673009713, 228.62763222672373, 255.71944896860265, 276.49287030684445,
                297.11096726546964, 322.2785541384614, 358.74827673009713, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ].to_vec(),
            ascmc: [
                48.62763222672374, 297.11096726546964, 299.1619604398136, 201.66776865408326,
                31.307700535881203, 22.55507848219912, 52.35280295047079, 202.55507848219912, 0.0, 0.0,
            ],
            result: 0,
        };

        let actual_result = houses_with_flag(date, flag, lat, lng, house_system);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    pub fn test_house_calculation_extended_with_speeds() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;
        set_ephe_path(EPHEMERIS_PATH);

        let expected_result = HouseCalculationResult {
            cusps: [
                48.62763222672374, 75.71944896860263, 96.49287030684448, 117.11096726546964,
                142.2785541384614, 178.74827673009713, 228.62763222672373, 255.71944896860265,
                276.49287030684445, 297.11096726546964, 322.2785541384614, 358.74827673009713,
            ],
            ascmc: [
                48.62763222672374, 297.11096726546964, 299.1619604398136, 201.66776865408326,
                31.307700535881203, 22.55507848219912, 52.35280295047079, 202.55507848219912,
            ],
            cusp_speeds: [
                0.0, 362.56618362266704, 326.3715539865318, 344.12778754362796, 416.59494083438483, 539.1280261881008,
                507.3817187152055, 362.56618362266704, 326.3715539865318, 344.12778754362796,
                416.59494083438483, 539.1280261881008,
            ],
            ascmc_speeds: [
                507.3817187152055, 344.12778754362796, 360.98564733498665, 267.1306504408959,
                376.64136009832293, 277.88707643085064, 520.5003314381933, 277.88707643085064,
            ],
        };

        let actual_result = calculate_houses_extended_with_speeds(date, 0, lat, lng, house_system);

        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_result);
    }

    #[test]
    pub fn test_house_calculation_extended_with_speeds_sidereal_calculations() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let house_system = HouseSystems::PLACIDUS;
        set_ephe_path(EPHEMERIS_PATH);
        set_sidereal_mode(Ayanamshas::GALACTIC_CENTER_MULA_WILHELM);
        let flag = CalculationFlags::SIDEREAL_POSITIONS;

        let expected_result = HouseCalculationResult {
            cusps: [
                28.225351258990685, 55.31716800086958, 76.09058933911143, 96.70868629773659,
                121.87627317072835, 158.34599576236408, 208.22535125899068, 235.3171680008696,
                256.0905893391114, 276.70868629773656, 301.8762731707283, 338.34599576236405,
            ],
            ascmc: [
                28.225351258990685, 276.70868629773656, 299.1619604398136, 181.2654876863502,
                10.905419568148147, 2.152797514466066, 31.950521982737733, 182.15279751446607,
            ],
            cusp_speeds: [
                0.0, 362.56618362266704, 326.3715539865318, 344.12778754362796, 416.59494083438483, 539.1280261881008,
                507.3817187152055, 362.56618362266704, 326.3715539865318, 344.12778754362796,
                416.59494083438483, 539.1280261881008,
            ],
            ascmc_speeds: [
                507.3817187152055, 344.12778754362796, 360.98564733498665, 267.1306504408959,
                376.64136009832293, 277.88707643085064, 520.5003314381933, 277.88707643085064,
            ],
        };

        let actual_result = calculate_houses_extended_with_speeds(
            date,
            flag,
            lat,
            lng,
            house_system
        );

        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_result);
    }

    #[test]
    fn test_calculate_houses_extended_with_speeds_error_case_when_calling_not_implemented_house_system() {
        let lat = 43.084128;
        let lng = 25.5919228;
        let date = get_test_date_time();
        let iflag = 0;
        let hsys = 'G'; // Invalid house system

        let result = calculate_houses_extended_with_speeds(date, iflag, lat, lng, hsys);

        assert!(result.is_err(), "Expected an error but got Ok");
        if let Err(err_msg) = result {
            println!("Error message: {}", err_msg);
            assert!(!err_msg.is_empty(), "Error message should not be empty");
        }
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
            Calendar::Gregorian
        );
        jd.julian_day_ut
    }
}
