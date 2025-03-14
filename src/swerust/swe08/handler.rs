use chrono::{ DateTime, FixedOffset, NaiveDate, TimeZone, Utc };

use crate::raw::{ self, swe_revjul };
use crate::sweconst::Calendar;
use std::ffi::{ CStr, CString };

/*
 * 8. Date and time conversion functions
 */
pub fn julday(year: i32, month: i32, day: i32, hour: f64, calendar: Calendar) -> f64 {
    let result: f64 = unsafe { raw::swe_julday(year, month, day, hour, calendar as i32) };
    result
}

/// [0 -> jday / 1 -> utc]
#[derive(Debug, Clone, PartialEq)]
pub struct UtcTimeZoneResult {
    pub year: [i32; 2],
    pub month: [i32; 2],
    pub day: [i32; 2],
    pub hour: [i32; 2],
    pub min: [i32; 2],
    pub sec: [f64; 2],
}

pub fn utc_time_zone(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: f64,
    timezone: f64
) -> UtcTimeZoneResult {
    let mut year_out = [0; 2];
    let mut month_out = [0; 2];
    let mut day_out = [0; 2];
    let mut hour_out = [0; 2];
    let mut min_out = [0; 2];
    let mut sec_out = [0.0; 2];
    unsafe {
        let p_year_out = year_out.as_mut_ptr();
        let p_month_out = month_out.as_mut_ptr();
        let p_day_out = day_out.as_mut_ptr();
        let p_hour_out = hour_out.as_mut_ptr();
        let p_min_out = min_out.as_mut_ptr();
        let p_sec_out = sec_out.as_mut_ptr();
        raw::swe_utc_time_zone(
            year,
            month,
            day,
            hour,
            min,
            sec,
            timezone,
            p_year_out,
            p_month_out,
            p_day_out,
            p_hour_out,
            p_min_out,
            p_sec_out
        );
        UtcTimeZoneResult {
            year: year_out,
            month: month_out,
            day: day_out,
            hour: hour_out,
            min: min_out,
            sec: sec_out,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UtcToJdResult {
    pub julian_day_et: f64,
    pub julian_day_ut: f64,
    pub err: String, // To do in other file same struct
    pub result: i32,
}
pub fn utc_to_jd(
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    sec: f64,
    calendar: Calendar
) -> UtcToJdResult {
    let mut dret = [0.0; 2];
    let mut serr = [0; 255];
    unsafe {
        let p_dret = dret.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let result = raw::swe_utc_to_jd(
            year,
            month,
            day,
            hour,
            min,
            sec,
            calendar as i32,
            p_dret,
            p_serr
        );
        let s_serr = CString::from(CStr::from_ptr(p_serr)).to_str().unwrap().to_string();
        UtcToJdResult {
            julian_day_et: dret[0],
            julian_day_ut: dret[1],
            err: s_serr,
            result,
        }
    }
}

pub fn julian_to_utc(jd: f64) -> DateTime<Utc> {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0.0;

    unsafe {
        swe_revjul(jd, 1, &mut year, &mut month, &mut day, &mut hour);
    }
    convert_to_utc_time(year, month, day, hour)
}

pub fn julian_to_dt_with_offset(
    jd: f64,
    timezone_offset: f64
) -> Result<DateTime<FixedOffset>, String> {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0.0;

    unsafe {
        swe_revjul(jd, 1, &mut year, &mut month, &mut day, &mut hour);
    }
    convert_to_dt_with_offset(year, month as u32, day as u32, hour, timezone_offset)
}

pub fn convert_to_dt_with_offset(
    year: i32,
    month: u32,
    day: u32,
    hour: f64,
    timezone_offset: f64
) -> Result<DateTime<FixedOffset>, String> {
    // Extract integer and fractional part of the hour
    let hour_int = hour as u32;
    let minute = ((hour - (hour_int as f64)) * 60.0) as u32;
    let second = (((hour - (hour_int as f64)) * 60.0 - (minute as f64)) * 60.0) as u32;

    // Create a NaiveDateTime (which is timezone-independent)
    let naive_date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(||
        "Invalid date".to_string()
    )?;
    let naive_datetime = naive_date
        .and_hms_opt(hour_int, minute, second)
        .ok_or_else(|| "Invalid time".to_string())?;

    // Convert to UTC DateTime
    let utc_datetime = Utc.from_utc_datetime(&naive_datetime);

    // Convert to local time using the provided offset (timezone_offset in hours)
    let timezone = FixedOffset::east_opt((timezone_offset * 3600.0) as i32).ok_or_else(||
        "Invalid timezone offset".to_string()
    )?;

    Ok(utc_datetime.with_timezone(&timezone))
}

pub fn convert_to_utc_time(year: i32, month: i32, day: i32, hour: f64) -> DateTime<Utc> {
    // Extract integer and fractional part of the hour
    let hour_int = hour as u32;
    let minute = ((hour - (hour_int as f64)) * 60.0) as u32;
    let second = (((hour - (hour_int as f64)) * 60.0 - (minute as f64)) * 60.0) as u32;

    // Create a NaiveDateTime (which is timezone-independent)
    let naive_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
        .ok_or_else(|| "Invalid date".to_string())
        .unwrap();
    let naive_datetime = naive_date
        .and_hms_opt(hour_int, minute, second)
        .ok_or_else(|| "Invalid time".to_string())
        .unwrap();

    Utc.from_utc_datetime(&naive_datetime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_utc_time() {
        let year = 2025;
        let month = 3;
        let day = 14;
        let hour = 12;
        let min = 6;
        let sec = 0.0;
        let timezone_offset = 2.0;

        let expected_result = UtcTimeZoneResult {
            year: [2025, 0],
            month: [3, 0],
            day: [14, 0],
            hour: [10, 0],
            min: [5, 0],
            sec: [59.99999999999872, 0.0],
        };

        let actual_result = utc_time_zone(year, month, day, hour, min, sec, timezone_offset);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    pub fn test_utc_to_jd() {
        let year = 2025;
        let month = 3;
        let day = 14;
        let hour = 10;
        let min = 6;
        let sec = 0.0;
        let expected_result = UtcToJdResult {
            julian_day_et: 2460748.9216340743,
            julian_day_ut: 2460748.9208356882,
            err: "".to_owned(),
            result: 0,
        };

        let actual_result = utc_to_jd(year, month, day, hour, min, sec, Calendar::Gregorian);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    pub fn test_convert_to_local_time() {
        let year = 2025;
        let month = 3;
        let day = 14;
        let hour = 10.1001;
        let expected_result = Utc.with_ymd_and_hms(
            year,
            month as u32,
            day as u32,
            10,
            6,
            0
        ).unwrap();

        let actual_result = convert_to_utc_time(year, month, day, hour);

        assert_eq!(actual_result, expected_result);
    }
}
