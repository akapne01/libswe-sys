/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (s.bressani@bluewin.ch) extended by Agnese
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
use std::env;
use std::path::Path;

#[allow(dead_code)]
fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Check if we're on Windows with MSVC
    let is_msvc = env
        ::var("TARGET")
        .map(|t| t.contains("msvc"))
        .unwrap_or(false);

    if is_msvc {
        // Set MSVC-specific environment variables
        env::set_var("CFLAGS", "/W3 /MD /Zi /wd4100 /wd4189 /wd4244 /wd4267 /wd4996");
    }

    let mut build = cc::Build::new();

    // Add all source files
    let files = [
        "src/swisseph/swecl.c",
        "src/swisseph/swedate.c",
        "src/swisseph/swehel.c",
        "src/swisseph/swehouse.c",
        "src/swisseph/swejpl.c",
        "src/swisseph/swemmoon.c",
        "src/swisseph/swemplan.c",
        "src/swisseph/swepcalc.c",
        "src/swisseph/sweph.c",
        "src/swisseph/swephlib.c",
    ];

    for file in files.iter() {
        build.file(Path::new(&dir).join(file));
    }

    // Only add flags if not MSVC (let environment variables handle MSVC)
    if !is_msvc {
        build
            .flag("-Wall")
            .flag("-g")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-but-set-parameter")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-function")
            .flag("-Wno-sign-compare")
            .flag("-Wno-unused-variable");
    }

    build.compile("swe");
}
