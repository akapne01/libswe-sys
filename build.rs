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

    for file in files.iter() {
        build.file(Path::new(&dir).join(file));
    }

    // Configure based on target platform
    let target = env::var("TARGET").unwrap();

    // #### TARGET PLATFORM ###: x86_64-pc-windows-msvc
    println!("#### TARGET PLATFORM ###: {}", target);

    if target.contains("msvc") {
        // MSVC-specific configuration
        println!("### Building Windows ###");
        build
            .flag("/W3") // Warning level 3
            .flag("/MD") // Use multithreaded DLL runtime
            .flag("/Zi") // Generate debug info
            .flag("/FS") // force MSVC to serialize access to PDB
            .flag("/wd4100") // Disable 'unreferenced formal parameter'
            .flag("/wd4189") // Disable 'local variable initialized but not referenced'
            .flag("/wd4244") // Disable 'conversion' warnings
            .flag("/wd4267") // Disable 'conversion from size_t'
            .flag("/wd4996"); // Disable deprecated function warnings
    } else {
        // GCC/Clang configuration
        println!("### Building Other Platform ###");
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

    // Compile the library
    build.compile("swe");

    // Print some debug info
    println!("### cargo:warning=Building for target: {}", target);
    println!(
        "### cargo:warning=Compiler: {:?}",
        build.get_compiler().path()
    );
}
