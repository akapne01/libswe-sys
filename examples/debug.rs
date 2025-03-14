extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate strum;

use libswe_sys::constants::EPHEMERIS_PATH;
use strum::IntoEnumIterator;

use libswe_sys::sweconst::{ Angle, Bodies, Calendar, House, Object, ObjectType, OptionalFlag };
use libswe_sys::swerust::{
    handler_swe02,
    handler_swe03,
    handler_swe07,
    handler_swe08,
    handler_swe14,
};
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Data {
    year: i32,
    month: i32,
    day: i32,
    hourf64: f64,
    hour: i32,
    min: i32,
    sec: f64,
    lat: f64,
    lng: f64,
}

fn main() {
    println!("Swissephem C -> Rust");
    
    handler_swe02::set_ephe_path(&EPHEMERIS_PATH);
    println!("Set the path of ephemeris to: {}", &EPHEMERIS_PATH);
    println!("Version swephem: {}", handler_swe02::version());
    println!("Get path of library: {}", handler_swe02::get_library_path());

    const PATH: &str = "examples/data.json";
    let mut s = String::new();
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push(PATH);
    File::open(file_path.as_path()).unwrap().read_to_string(&mut s).unwrap();
    let data: Data = serde_json::from_str(&s).unwrap();
    println!("Data: {:?}", data);
    let julday: f64 = handler_swe08::julday(
        data.year,
        data.month,
        data.day,
        data.hourf64,
        Calendar::Gregorian
    );
    println!("Get julday: {:?}", julday);

    let mut object: Vec<Object> = Vec::new();
    let mut calc: handler_swe03::CalcUtResult;
    for bodies in Bodies::iter() {
        if
            bodies.clone().object_type() == ObjectType::PlanetOrStar ||
            bodies.clone().object_type() == ObjectType::Fiction
        {
            calc = handler_swe03::calc_ut(julday, bodies.clone(), OptionalFlag::Speed as i32);
            object.push(
                Object::new(
                    bodies.clone(),
                    bodies.clone().into(),
                    bodies.clone().object_type(),
                    calc.longitude,
                    calc.latitude,
                    calc.speed_longitude
                )
            );
        }
    }

    for o in object {
        println!("{:?}", o);
    }

    let pheno_ut: handler_swe07::PhenoUtResult = handler_swe07::pheno_ut(
        julday,
        Bodies::Sun,
        OptionalFlag::Speed as i32
    );
    println!("PhenoUt: {:?}", pheno_ut);

    // let hsys = HouseSystem::Placidus;
    let name = handler_swe14::house_name('P');
    println!("Hsys: {}", name);

    let utc_time_zone: handler_swe08::UtcTimeZoneResult = handler_swe08::utc_time_zone(
        data.year,
        data.month,
        data.day,
        data.hour,
        data.min,
        data.sec,
        2.0
    );
    println!("utc_time_zone: {:?}", utc_time_zone);

    let utc_to_jd: handler_swe08::UtcToJdResult = handler_swe08::utc_to_jd(
        utc_time_zone.year[0],
        utc_time_zone.month[0],
        utc_time_zone.day[0],
        utc_time_zone.hour[0],
        utc_time_zone.min[0],
        utc_time_zone.sec[0],
        /*utc_time_zone.year[1],
        utc_time_zone.month[1],
        utc_time_zone.day[1],
        utc_time_zone.hour[1],
        utc_time_zone.min[1],
        utc_time_zone.sec[1],*/
        Calendar::Gregorian
    );
    println!("utc_to_jd: {:?}", utc_to_jd);

    // Whole signs
    let result_w = handler_swe14::houses(utc_to_jd.julian_day_ut, data.lat, data.lng, 'W');
    //println!("House object: {:?}", result);
    let mut house2: Vec<House> = Vec::new();
    for (i, res) in result_w.clone().cusps.iter().enumerate() {
        if i > 0 {
            // No angle calculation when Nothing
            let angle = Angle::Nothing;
            house2.push(House::new(i as i32, res.clone(), angle));
            if i + 1 > 12 {
                break;
            }
        }
    }

    for h in house2 {
        println!("{:?}", h);
    }
    println!("House (wohle signs): {:?}", result_w.clone());

    // Wohle Signs
    let result = handler_swe14::houses(utc_to_jd.julian_day_ut, data.lat, data.lng, 'P');
    //println!("House object: {:?}", result);
    let mut house: Vec<House> = Vec::new();
    for (i, res) in result.clone().cusps.iter().enumerate() {
        if i > 0 {
            let angle;
            /*
            if result.clone().ascmc[0] == res.clone() {
                angle = Angle::Asc;
            }
            if result.clone().ascmc[1] == res.clone() {
                angle = Angle::Fc;
            }
            if result.clone().ascmc[2] == res.clone() {
                angle = Angle::Desc;
            }
            if result.clone().ascmc[3] == res.clone() {
                angle = Angle::Mc;
            }*/
            // This is tested with Placidus only
            // the line above ascmc[?] don't work for Desc and Mc
            angle = match i {
                1 => Angle::Asc,
                4 => Angle::Fc,
                7 => Angle::Desc,
                10 => Angle::Mc,
                _ => Angle::Nothing,
            };
            house.push(House::new(i as i32, res.clone(), angle));
            if i + 1 > 12 {
                break;
            }
        }
    }

    for h in house {
        println!("{:?}", h);
    }
    println!("House (Placidus): {:?}", result.clone());

    // Fortuna part
    let calcfp = handler_swe03::calc_ut_fp(
        julday,
        data.lat,
        data.lng,
        'P',
        OptionalFlag::Speed as i32,
    );
    println!("Fortuna Part: {}", calcfp.longitude);

    println!("Exit and free memory swephem");
    handler_swe02::close();
}
