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
    /*
    let host = std::env::var("HOST").unwrap();
    let target = std::env::var("TARGET").unwrap();
    let mut path_header = "/usr/include".to_string();
    if target.contains("wasm32") {
        if host.contains("darwin") {
            // brew install llvm
            std::env::set_var("CC", "/usr/local/opt/llvm/bin/clang");
            std::env::set_var("AR", "/usr/local/opt/llvm/bin/llvm-ar");
            path_header = "/usr/local/opt/llvm/include".to_string();
        }
    }*/
    /*
     * Old Way
     * Not work with cargo
     *
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir)
                .join("src/swisseph/src/build")
                .display()
        );
    */

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut build = cc::Build::new();

    build
        .flag("-g") // keep debug info
        .file(Path::new(&dir).join("src/swisseph/swecl.c"))
        .file(Path::new(&dir).join("src/swisseph/swedate.c"))
        .file(Path::new(&dir).join("src/swisseph/swehel.c"))
        .file(Path::new(&dir).join("src/swisseph/swehouse.c"))
        .file(Path::new(&dir).join("src/swisseph/swejpl.c"))
        .file(Path::new(&dir).join("src/swisseph/swemmoon.c"))
        .file(Path::new(&dir).join("src/swisseph/swemplan.c"))
        .file(Path::new(&dir).join("src/swisseph/swepcalc.c"))
        .file(Path::new(&dir).join("src/swisseph/sweph.c"))
        .file(Path::new(&dir).join("src/swisseph/swephlib.c"));

    // Only add GCC/Clang flags on non-MSVC targets
    if !cfg!(target_env = "msvc") {
        build
            .flag("-Wall")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-but-set-parameter")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-function")
            .flag("-Wno-sign-compare");
    }

    build.compile("swe");
}
