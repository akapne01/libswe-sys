use crate::swerust::handler_swe14::house_name;

pub static EPHEMERIS_PATH: &str =
    "/home/kaliorion/dev/sandbox/swisseph/ephe/www.astro.com/ftp/swisseph/ephe";

// Calculation options: https://www.astro.com/swisseph/swephprg.htm
pub struct CalculationFlags;

impl CalculationFlags {
    pub const JPL_EPHEMERIS: i32 = 1; // use JPL ephemeris
    pub const SWISS_EPHEMERIS: i32 = 2; // use SWISSEPH ephemeris, default
    pub const MOSHIER_EPHEMERIS: i32 = 4; // use Moshier ephemeris
    pub const HELIOCENTRIC_POSITIONS: i32 = 8; // return heliocentric position
    pub const TRUE_POSITIONS: i32 = 16; // return true geometric positions, not apparent
    pub const NO_PROCESSION_J2000: i32 = 32; // no precession, i.e. give J2000 equinox
    pub const MEAN_EQUINOX_OF_DATE_NO_NUTATION: i32 = 64; // no nutation, i.e. mean equinox of date
    pub const SPEED_PRECISION: i32 = 256; // high precision speed (analyt. comp.)
    pub const NO_GRAVITY_DEFLECTION: i32 = 512; // turn off gravitational deflection
    pub const NO_ABERRATION: i32 = 1024; // turn off 'annual' aberration of light
    pub const ASTROMETRIC_POSITIONS: i32 = 1536; // astrometric positions. With light-time, without aberration and light deflection.
    pub const EQUATORIAL_POSITIONS: i32 = 2048; // equatorial positions are wanted (incl. declination)
    pub const CARTESIAN_COORDINATES: i32 = 4096; // cartesian, not polar, coordinates
    pub const RADIAN_COORDINATES: i32 = 8192; // coordinates in radians, not degrees
    pub const BARYCENTRIC_POSITIONS: i32 = 16384; // barycentric positions
    pub const TOPOCENTRIC_POSITIONS: i32 = 32768; // topocentric positions
    pub const TROPICAL_POSITIONS: i32 = 0; // Default 
    pub const SIDEREAL_POSITIONS: i32 = 65536; // sidereal positions
    pub const ICRS: i32 = 131072; // ICRS (DE406 reference frame)
    pub const JPL_HORIZONS: i32 = 262144; // reproduce JPL Horizons 1962 - today to 0.002 arcsec.
    pub const APPROXIMATE_JPL_HORIZONS: i32 = 524288; // approximate JPL Horizons 1962 - today
    pub const CENTER_BODY: i32 = 1048576; // calculate position of center of body (COB) of planet, not barycenter of its system
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ayanamsha;

impl Ayanamsha {
    pub const FAGAN_BRADLEY: i32 = 0; // Fagan/Bradley
    pub const LAHIRI: i32 = 1; // Lahiri
    pub const DE_LUCE: i32 = 2; // De Luce
    pub const RAMAN: i32 = 3; // Raman
    pub const USHA_SHASHI: i32 = 4; // Usha/Shashi
    pub const KRISHNAMURTI: i32 = 5; // Krishnamurti
    pub const DJWHAL_KHUL: i32 = 6; // Djwhal Khul
    pub const YUKTESHWAR: i32 = 7; // Yukteshwar
    pub const JN_BHASIN: i32 = 8; // J.N. Bhasin
    pub const BABYLONIAN_KUGLER_1: i32 = 9; // Babylonian/Kugler 1
    pub const BABYLONIAN_KUGLER_2: i32 = 10; // Babylonian/Kugler 2
    pub const BABYLONIAN_KUGLER_3: i32 = 11; // Babylonian/Kugler 3
    pub const BABYLONIAN_HUBER: i32 = 12; // Babylonian/Huber
    pub const BABYLONIAN_ETA_PISCIUM: i32 = 13; // Babylonian/Eta Piscium
    pub const BABYLONIAN_ALDEBARAN_15_TAU: i32 = 14; // Babylonian/Aldebaran = 15 Tau
    pub const HIPPARCHOS: i32 = 15; // Hipparchos
    pub const SASSANIAN: i32 = 16; // Sassanian
    pub const GALACTIC_CENTER_0_SAG: i32 = 17; // Galactic Center = 0 Sag
    pub const J2000: i32 = 18; // J2000
    pub const J1900: i32 = 19; // J1900
    pub const B1950: i32 = 20; // B1950
    pub const SURYASIDDHANTA: i32 = 21; // Suryasiddhanta
    pub const SURYASIDDHANTA_MEAN_SUN: i32 = 22; // Suryasiddhanta, mean Sun
    pub const ARYABHATA: i32 = 23; // Aryabhata
    pub const ARYABHATA_MEAN_SUN: i32 = 24; // Aryabhata, mean Sun
    pub const SS_REVATI: i32 = 25; // SS Revati
    pub const SS_CITRA: i32 = 26; // SS Citra
    pub const TRUE_CITRA: i32 = 27; // True Citra
    pub const TRUE_REVATI: i32 = 28; // True Revati
    pub const TRUE_PUSHYA: i32 = 29; // True Pushya (PVRN Rao)
    pub const GALACTIC_CENTER_GIL_BRAND: i32 = 30; // Galactic Center (Gil Brand)
    pub const GALACTIC_EQUATOR_IAU1958: i32 = 31; // Galactic Equator (IAU1958)
    pub const GALACTIC_EQUATOR_TRUE: i32 = 32; // Galactic Equator
    pub const GALACTIC_EQUATOR_MID_MULA: i32 = 33; // Galactic Equator mid-Mula
    pub const GALACTIC_ALIGNMENT_SKYDRAM_MARDYKS: i32 = 34; // Skydram (Mardyks)
    pub const TRUE_MULA_CHANDRA_HARI: i32 = 35; // True Mula (Chandra Hari)
    pub const GALACTIC_CENTER_MULA_WILHELM: i32 = 36; // Dhruva/Gal.Center/Mula (Wilhelm)
    pub const ARYABHATA_522: i32 = 37; // Aryabhata 522
    pub const BABYLONIAN_BRITTON: i32 = 38; // Babylonian/Britton
    pub const VEDIC_SHEORAN: i32 = 39; // \"Vedic\"/Sheoran
    pub const GALACTIC_CENTER_COCHRANE_0_CAP: i32 = 40; // \"Vedic\"/Sheoran
    pub const GALACTIC_EQUATOR_FIORENZA: i32 = 41; // Galactic Equator (Fiorenza)
    pub const VETTIUS_VALENS_MOON: i32 = 42; // Galactic Equator (Fiorenza)
    pub const LAHIRI_1940: i32 = 43; // Lahiri 1940
    pub const LAHIRI_VP285: i32 = 44; // Lahiri VP285
    pub const KRISHNAMURTI_SENTHILATHIBAN: i32 = 45; // Krishnamurti-Senthilathiban
    pub const LHIRI_ICRC: i32 = 45; // Lahiri ICRC
}

#[derive(Debug, Clone, PartialEq)]
pub struct HouseSystems;

/// 'G': GAUQUELIN_SECTORS is not included since it returns array of 36 and currently
/// it is not implemented.
impl HouseSystems {
    pub const PLACIDUS: char = 'P';
    pub const KOCH: char = 'K';
    pub const PORPHYRIUS: char = 'O';
    pub const REGIOMONTANUS: char = 'R';
    pub const CAMPANUS: char = 'C';
    pub const EQUAL: char = 'E'; // Equal (cusp 1 is Ascendant)
    pub const WHOLE_SIGN: char = 'W';
    pub const ALCABITUS: char = 'B';
    pub const EQUAL_MC: char = 'D'; // Equal MC (cusp 10 is MC)
    pub const CARTHER_POLI_EQUATORIAL: char = 'F'; // Carter "Poli-Equatorial"
    pub const HORIZONTAL_SYSTEM: char = 'H'; // Azimuthal or horizontal system
    pub const SUNSHINE_TREINDL: char = 'I'; // Sunshine (Makransky, solution Treindl)
    pub const SUNSHINE_MAKRANSKY: char = 'i'; // Sunshine (Makransky, solution Makransky)
    pub const KRUSINSKI_PISA_GOELZER: char = 'U'; // Krusinski-Pisa-Goelzer Meridian system -> axial rotation
    pub const MORINUS: char = 'M';
    pub const POLICH_PAGE_TOPOCENTRIC: char = 'T'; // Polich/Page (topocentric system)
    pub const PULLEN_SINUSOIDAL_DELTA: char = 'L'; // Pullen SD (sinusoidal delta): ex Neo-Porphyry
    pub const PULLEN_SINUSOIDAL_RATIO: char = 'Q'; // Pullen SR (sinusoidal ratio)
    pub const SRIPATI_TOPOCENTRIC: char = 'S'; // Sripati Topocentric system -> Polich/Page
    pub const VEHLOW_EQUAL: char = 'V'; // Vehlow equal (Asc. in middle of house 1)
    pub const APC_HOUSES: char = 'Y'; // APC houses
    pub const MERIDIAN_HOUSES: char = 'X'; // Axial rotation system / Meridian system / Zariel

    pub fn get_house_system_name(house_system: char) -> String {
        let name = house_name(house_system);
        if house_system == HouseSystems::WHOLE_SIGN {
            return name.replace("equal/ ", "");
        }
        name
    }
}
