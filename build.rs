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
    for f in files.iter() {
        build.file(Path::new(&dir).join(f));
    }

    // Get the compiler
    let compiler = build.get_compiler();

    // Configure flags based on compiler
    if compiler.is_like_msvc() {
        // MSVC flags
        build.flag("/W3"); // warning level 3
        build.flag("/Zi"); // debug info
        build.flag("/MD"); // dynamic CRT
        // Optional: disable specific warnings
        build.flag("/wd4100"); // unreferenced formal parameter
        build.flag("/wd4189"); // local variable set but not used
        build.flag("/wd4244"); // conversion warnings
    } else {
        // GCC/Clang flags
        build
            .flag("-Wall")
            .flag("-g")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-but-set-parameter")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-function")
            .flag("-Wno-sign-compare");
    }

    // Compile the library
    build.compile("swe");
}
