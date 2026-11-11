extern crate math;
use crate::raw;
use crate::sweconst::Signs;
use math::round;
use strum::IntoEnumIterator;
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;

/*
 * 17. Auxilliary functions
 */
pub fn degnorm(x: f64) -> f64 {
    unsafe { raw::swe_degnorm(x) }
}

pub fn radnorm(x: f64) -> f64 {
    unsafe { raw::swe_radnorm(x) }
}

#[derive(Debug, Clone)]
pub struct SplitDegResult {
    pub print: String,
    pub deg: i32,
    pub min: i32,
    pub sec: i32,
    pub cdegfr: f64,
    pub sign: Signs,
}

/// float to deg
/// isgn return me always 0 ? I compute this value manualy
pub fn split_deg(ddeg: f64, roundflag: i32) -> SplitDegResult {
    let sign_calc = round::floor(ddeg / 30.0, 0) as usize;
    let house_calc = round::floor(ddeg / 30.0, 0);
    let long_30 = (house_calc * 30.0) - ddeg;

    let mut deg: i32 = 0;
    let mut min: i32 = 0;
    let mut sec: i32 = 0;
    let mut cdegfr: f64 = 0.0;
    let mut isgn: i32 = 0;
    unsafe {
        raw::swe_split_deg(
            long_30,
            roundflag,
            &mut deg,
            &mut min,
            &mut sec,
            &mut cdegfr,
            &mut isgn,
        );
    }
    let print = format!("{}°{:02}'{:02}", deg.abs(), min, sec);
    SplitDegResult {
        print,
        deg,
        min,
        sec,
        cdegfr,
        sign: Signs::iter().nth(sign_calc).unwrap_or(Signs::Aries),
    }
}
