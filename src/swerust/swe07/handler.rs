use crate::raw::{ self, swe_rise_trans };
use crate::sweconst::Bodies;
use std::ffi::{ CStr, CString };
use std::ptr;

pub struct CalculationMethodsRiseTransit;

impl CalculationMethodsRiseTransit {
    pub const RISE: i32 = 1;
    pub const SET: i32 = 2;
    pub const UPPER_MERIDIAN_TRANSIT: i32 = 4; // upper meridian transit (southern for northern geo. latitudes)
    pub const LOWER_MERIDIAN_TRANSIT: i32 = 8; // lower meridian transit (northern, below the horizon)
    // Below calculation methods can be used in conjunction with RISE & SET.
    pub const DISC_CENTER: i32 = 256; // for rising or setting of disc center
    pub const DISC_BOTTOM: i32 = 8192; // for rising or setting of lower limb of disc
    pub const GEOCENTRIC: i32 = 128; // use geocentric (rather than topocentric) position of object and ignore its ecliptic latitude
    pub const NO_REFRACTION: i32 = 512; // if refraction is not to be considered
    pub const CIVIL_TWILIGHT: i32 = 1024; // in order to calculate civil twilight
    pub const NAUTICAL_TWILIGHT: i32 = 2048; // in order to calculate nautical twilight
    pub const ASTRONOMICAL_TWILIGHT: i32 = 4096; // in order to calculate astronomical twilight
    pub const FIXED_DISC_SIZE: i32 = 16 * 1024; // neglect the effect of distance on disc size
    // Consider Sun rising and setting when the center of solar disk is exactly over horizon. Also
    // ignores atmospheric refraction and uses geocentric rather than topocentric calculations.
    pub const HINDU_RISING: i32 = 256 + 512 + 128; // Composition of DISC_CENTER + NO_REFRACTION + GEOCENTRIC risings according to Hindu astrology
}

/*
 * 7. Eclipses, risings, settings, meridian transits, planetary phenomena
 */
#[derive(Debug)]
pub struct PhenoUtResult {
    pub phase_angle: f64,
    pub phase_illuminated: f64,
    pub elongation_of_planet: f64,
    pub apparent_dimaeter_of_disc: f64,
    pub apparent_magnitude: f64,
    pub status: i32,
    pub serr: String,
}

pub fn pheno_ut(tjd_ut: f64, ipl: Bodies, iflag: i32) -> PhenoUtResult {
    let mut attr: [f64; 20] = [0.0; 20];
    let mut serr = [0; 255];
    let result = unsafe {
        let p_attr = attr.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let status = raw::swe_pheno_ut(tjd_ut, ipl as i32, iflag, p_attr, p_serr);
        let s_serr = CString::from(CStr::from_ptr(p_serr)).to_str().unwrap().to_string();
        PhenoUtResult {
            phase_angle: attr[0],
            phase_illuminated: attr[1],
            elongation_of_planet: attr[2],
            apparent_dimaeter_of_disc: attr[3],
            apparent_magnitude: attr[4],
            serr: s_serr,
            status,
        }
    };
    result
}

/// Calculates rising, setting and meridian transits
pub fn rise_trans(
    tjd_ut: f64,
    planet: Bodies,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    calculation_flag: i32,
    calculation_method: i32
) -> Result<f64, String> {
    let geopos = [longitude, latitude, altitude]; // Observer's position
    let mut tret = 0.0; // Will store rise and set times
    let error_buffer = [0i8; 256]; // Error buffer

    unsafe {
        let pointer_to_error_buffer = error_buffer.as_ptr() as *mut i8;

        swe_rise_trans(
            tjd_ut,
            planet as i32,
            ptr::null(),
            calculation_flag,
            calculation_method,
            geopos.as_ptr(),
            0.0, // estimated by underlying C function
            0.0, // estimated by underlying C function
            &mut tret,
            pointer_to_error_buffer
        );

        let error_buffer_str = CString::from(CStr::from_ptr(pointer_to_error_buffer))
            .to_str()
            .unwrap()
            .to_string();
        if !error_buffer_str.is_empty() {
            return Err(error_buffer_str);
        }
    }

    Ok(tret)
}

/// Returns rising and setting times
pub fn calculate_rise_and_set(
    julian_day_utc: f64,
    planet: Bodies,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    calculation_flag: i32,
    additional_calculation_methods: i32
) -> Result<(f64, f64), String> {
    let rise_flag = CalculationMethodsRiseTransit::RISE + additional_calculation_methods;
    let set_flag = CalculationMethodsRiseTransit::SET + additional_calculation_methods;

    let rising_result = rise_trans(
        julian_day_utc,
        planet,
        latitude,
        longitude,
        altitude,
        calculation_flag,
        rise_flag
    );

    let setting_result = rise_trans(
        julian_day_utc,
        planet,
        latitude,
        longitude,
        altitude,
        calculation_flag,
        set_flag
    );

    let rising_jd = match rising_result {
        Ok(rising) => rising,
        Err(err) => {
            return Err(err);
        }
    };

    let setting_jd = match setting_result {
        Ok(setting) => setting,
        Err(err) => {
            return Err(err);
        }
    };

    Ok((rising_jd, setting_jd))
}
#[cfg(test)]
mod tests {
    use crate::{
        constants::CalculationFlags,
        sweconst::Calendar,
        swerust::handler_swe08::{ julian_to_dt_with_offset, utc_to_jd },
    };

    use super::*;

    #[test]
    pub fn test_calculate_rise_trans_sun_setting() {
        let julian_day_utc = 2460748.9208356882;
        let planet = Bodies::Sun;
        let latitude = 43.084128;
        let longitude = 25.5919228;
        let altitude = 265.0;
        let calculation_flag = CalculationFlags::SWISS_EPHEMERIS;
        let calculation_method = CalculationMethodsRiseTransit::SET;

        let result = rise_trans(
            julian_day_utc,
            planet,
            latitude,
            longitude,
            altitude,
            calculation_flag,
            calculation_method
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2460749.182606328);
    }

    #[test]
    pub fn test_calculate_rise_and_set_for_sun() {
        let julian_day = utc_to_jd(2025, 3, 14, 0, 0, 0.0, Calendar::Gregorian);
        let julian_day_utc = julian_day.julian_day_ut;
        let planet = Bodies::Sun;
        let latitude = 43.084128;
        let longitude = 25.5919228;
        let altitude = 265.0;
        let calculation_flag = CalculationFlags::SWISS_EPHEMERIS;
        let additional_calculation_methods = 0;
        let expected_result = (2460748.688380363, 2460749.182606324);

        let actual_result = calculate_rise_and_set(
            julian_day_utc,
            planet,
            latitude,
            longitude,
            altitude,
            calculation_flag,
            additional_calculation_methods
        );

        println!("Rising: {}", julian_to_dt_with_offset(2460748.688380363, 2.0).unwrap());
        println!("Setting: {}", julian_to_dt_with_offset(2460749.182606328, 2.0).unwrap());

        assert!(actual_result.is_ok());
        assert_eq!(actual_result.unwrap(), expected_result);
    }
}
