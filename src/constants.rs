pub static EPHEMERIS_PATH: &str =
    "/home/kaliorion/dev/sandbox/swisseph/ephe/www.astro.com/ftp/swisseph/ephe";

// Calculation options: https://www.astro.com/swisseph/swephprg.htm
pub struct CalculationFlags;

impl CalculationFlags {
    pub const JPL_EPHEMERIS: i32 = 1; // use JPL ephemeris
    pub const SWISS_EPHEMERIS: i32 = 2; // use SWISSEPH ephemeris, default
    pub const MOSHIER_EPHEMERIS: i32 = 4; // use Moshier ephemeris
    pub const HELIOCENTRIC_POSITIONS: i32 = 8; // return heliocentric position
    pub const TRUE_POSITIONS: i32 = 16; // return true positions, not apparent
    pub const NO_PROCESSION: i32 = 32; // no precession, i.e. give J2000 equinox
    pub const MEAN_EQUINOX_OF_DATE: i32 = 64; // no nutation, i.e. mean equinox of date
    pub const SPEED_PRECISION: i32 = 256; // high precision speed (analyt. comp.)
    pub const NO_GRAVITY_DEFLECTION: i32 = 512; // turn off gravitational deflection
    pub const NO_ABERRATION: i32 = 1024; // turn off 'annual' aberration of light
    pub const ASTROMETRIC_POSITIONS: i32 = 1536; // astrometric positions
    pub const EQUATORIAL_POSITIONS: i32 = 2048; // equatorial positions are wanted (incl. declination)
    pub const CARTESIAN_COORDINATES: i32 = 4096; // cartesian, not polar, coordinates
    pub const RADIAN_COORDINATES: i32 = 8192; // coordinates in radians, not degrees
    pub const BARYCENTRIC_POSITIONS: i32 = 16384; // barycentric positions
    pub const TOPOCENTRIC_POSITIONS: i32 = 32768; // topocentric positions
    pub const SIDEREAL_POSITIONS: i32 = 65536; // sidereal positions
    pub const ICRS: i32 = 131072; // ICRS (DE406 reference frame)
    pub const JPL_HORIZONS: i32 = 262144; // reproduce JPL Horizons 1962 - today to 0.002 arcsec.
    pub const APPROXIMATE_JPL_HORIZONS: i32 = 524288; // approximate JPL Horizons 1962 - today
    pub const CENTER_BODY: i32 = 1048576; // calculate position of center of body (COB) of planet, not barycenter of its system
}
