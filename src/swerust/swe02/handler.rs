use crate::raw;
use crate::swerust::ffi_util::{buf_to_string, AS_MAXCH};
use std::ffi::CString;
use std::os::raw::c_char;

/*
 * 2. The Ephemeris file related functions
 */

/// Set the path of ephemeris
pub fn set_ephe_path(path: &str) {
    if path.len() > 255 {
        panic!("swe 2.1 -> set_ephe_path -> path to long");
    }
    let c_str = CString::new(path).unwrap();
    let path_final: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_ephe_path(path_final);
    }
}

/// Close swiss ephemeris, free memory
pub fn close() {
    unsafe { raw::swe_close() }
}

/// Set the path of ephemeris for working with JPL file
pub fn set_jpl_file(fname: &str) {
    if fname.len() > 255 {
        panic!("swe 2.3 -> set_jpl_file -> fname to long");
    }
    let c_str = CString::new(fname).unwrap();
    let fname_final: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        raw::swe_set_jpl_file(fname_final);
    }
}

/// Get version of swiss ephemeris
pub fn version() -> String {
    let mut buf = [0 as c_char; AS_MAXCH];
    unsafe {
        raw::swe_version(buf.as_mut_ptr());
    }
    buf_to_string(&buf)
}

/// Get library path (dll)
pub fn get_library_path() -> String {
    let mut buf = [0 as c_char; AS_MAXCH];
    unsafe {
        raw::swe_get_library_path(buf.as_mut_ptr());
    }
    buf_to_string(&buf)
}
