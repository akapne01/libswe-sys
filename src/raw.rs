use std::os::raw::{c_char, c_double, c_int};
// Interface https://www.astro.com/ftp/swisseph/doc/swephprg.htm#_Toc19111156
#[link(name = "swe")]
extern "C" {
    /*
     * 2. The Ephemeris file related functions
     */

    /// void swe_set_ephe_path(
    ///     char *path);
    pub fn swe_set_ephe_path(path: *const c_char);

    /// /* close Swiss Ephemeris */
    /// void swe_close(
    ///     void);
    pub fn swe_close();

    /// /* set name of JPL ephemeris file */
    /// void swe_set_jpl_file(
    ///     char *fname);
    pub fn swe_set_jpl_file(fname: *const c_char);

    /// /* find out version number of your Swiss Ephemeris version */
    /// char *swe_version(
    ///     char *svers);
    /// /* svers is a string variable with sufficient space to contain the
    /// version number (255 char) */
    pub fn swe_version(s_version: *mut c_char) -> *mut c_char;

    /// /* find out the library path of the DLL or executable */
    /// char *swe_get_library_path(
    ///     char *spath);
    /// /* spath is a string variable with sufficient space to contain the
    /// library path (255 char) */
    pub fn swe_get_library_path(spath: *mut c_char) -> *mut c_char;

    /*
     * 3. The functions swe_calc_ut() and swe_calc()
     * Before calling one of these functions or any other Swiss Ephemeris
     * function, it is strongly recommended to call the function
     * swe_set_ephe_path(). Even if you don’t want to set an ephemeris path and
     * use the Moshier ephemeris, it is nevertheless recommended to call
     * swe_set_ephe_path(NULL), because this function makes important
     * initializations. If you don’t do that, the Swiss Ephemeris may work but
     * the results may be not 100% consistent.
     */

    /// int swe_calc_ut(
    ///     double tjd_ut,
    ///     int ipl,
    ///     int iflag,
    ///     double* xx,
    ///     char* serr);
    ///
    /// tjd_ut    = Julian day, Universal Time
    /// ipl       = body number
    /// iflag     = a 32 bit integer containing bit flags that indicate what
    ///             kind of computation is wanted
    /// xx        = array of 6 doubles for longitude, latitude, distance, speed
    ///             in long., speed in lat., and speed in dist.
    /// serr[256] = character string to return error messages in case of error.
    pub fn swe_calc_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /*
     * 7. Eclipses, risings, settings, meridian transits, planetary phenomena
     */

    /// int32 swe_pheno_ut(
    ///     double tjd_ut,       /* time Jul. Day UT */
    ///     int32 ipl,           /* planet number */
    ///     int32 iflag,         /* ephemeris flag */
    ///     double *attr,        /* return array, 20 doubles, see below */
    ///     char *serr);         /* return error string */
    pub fn swe_pheno_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        atr: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /// Computes the times of rising, setting and meridian transits for all planets,
    /// asteroids, the moon, nd the fixed stars.
    /// Returns rising time of an object
    /// int32 swe_rise_trans(
    /// double tjd_ut,      /* search after this time (UT) */
    /// int32 ipl,               /* planet number, if planet or moon */
    /// char *starname,     /* star name, if star; must be NULL or empty, if ipl is used */
    /// int32 epheflag,     /* ephemeris flag */
    /// int32 rsmi,              /* integer specifying that rise, set, or one of the two meridian transits is wanted. see definition below */
    /// double *geopos,     /* array of three doubles containing
    ///                         * geograph. long., lat., height of observer */
    /// double atpress      /* atmospheric pressure in mbar/hPa */
    /// double attemp,      /* atmospheric temperature in deg. C */
    /// double *tret,            /* return address (double) for rise time etc. */
    /// char *serr);             /* return address for error message */
    ///
    /// Calculation types:
    ///  1 -> Rise
    ///  2 -> Set
    ///  4 -> Upper Meridian transit (southern for northern geo. latitudes)
    ///  8 -> Lower Meridian transit (norther, below the horizon)
    ///  256 -> For rising or setting of disc center
    ///  8192 -> For rising or setting of lower limb of disc
    ///  128 -> Use geocentric (rather than topocentric) position of object and ignore its ecliptic latitude
    ///  512 -> Don't consider refraction
    ///  1024 -> Calculate civil twilight
    ///  2048 -> Calculate nautical twilight
    ///  (16*1024) -> Neglect the effect of distance on disc size
    ///  0 -> Default: returns risings.
    /// The rising times depend on atmospheric pressure and temperature.
    /// atpress expects the atmospheric pressure in millibar (hectopascal); attemp the temperature in degrees Celsius.
    /// If atpress is given the value 0, the function estimates the pressure from the geographical altitude given in
    /// geopos[2] and attemp. If geopos[2] is 0, atpress will be estimated for sea level.
    ///
    /// Function return values are:
    /// 0 if a rising, setting or transit event was found;
    /// -1  if an error occurred (usually an ephemeris problem);
    /// -2  if a rising or setting event was not found because the object is circumpolar.
    /// The astronomical sunrise is defined as the time when the upper limb of the solar disk is seen appearing at the horizon.
    /// The astronomical sunset is defined as the moment the upper limb of the solar disk disappears below the horizon.
    /// The function swe_rise_trans() by default follows this definition of astronomical sunrises and sunsets.
    pub fn swe_rise_trans(
        tjd_ut: f64,         // Julian day number
        ipl: i32,            // Planet ID (SE_SUN, SE_MOON, etc.)
        starname: *const i8, // Empty for planets
        epheflag: i32,       // Ephemeris flag (e.g., SEFLG_SWIEPH)
        rsmi: i32,           // Calculation type (rise/set/transit)
        geopos: *const f64,  // Pointer to [longitude, latitude, altitude]
        atpress: f64,        // Atmospheric pressure
        attemp: f64,         // Atmospheric temperature
        tret: *mut f64,      // Output array for rise/set times
        serr: *mut i8,       // Error message buffer (if needed)
    ) -> i32; // Returns error code (0 if success)

    /*
     * 8. Date and time conversion functions
     */

    /// double swe_julday(
    ///     int year,
    ///     int month,
    ///     int day,
    ///     double hour,
    ///     int gregflag);
    pub fn swe_julday(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_double,
        gregflag: c_int,
    ) -> c_double;

    /// void swe_utc_time_zone(
    ///     int32 iyear, int32 imonth, int32 iday,
    ///     int32 ihour, int32 imin, double dsec,
    ///     double d_timezone,
    ///     int32 *iyear_out, int32 *imonth_out, int32 *iday_out,
    ///     int32 *ihour_out, int32 *imin_out, double *dsec_out);
    pub fn swe_utc_time_zone(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,
        ihour: c_int,
        imin: c_int,
        dsec: c_double,
        d_timezone: c_double,
        iyear_out: *mut c_int,
        imonth_out: *mut c_int,
        iday_out: *mut c_int,
        ihour_out: *mut c_int,
        imin_out: *mut c_int,
        dsec_out: *mut c_double,
    );

    /// int32 swe_utc_to_jd(
    /// int32 iyear, int32 imonth, int32 iday,
    /// int32 ihour, int32 imin, double dsec,          /* NOTE: second is a decimal */
    /// gregflag,            /* Gregorian calendar: 1, Julian calendar: 0 */
    /// dret                 /* return array, two doubles:
    ///                       * dret[0] = Julian day in ET (TT)
    ///                       * dret[1] = Julian day in UT (UT1) */
    /// serr);               /* error string */
    pub fn swe_utc_to_jd(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,
        ihour: c_int,
        imin: c_int,
        dsec: c_double,
        gregflag: c_int,
        dret: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /// swe_revjul() is the inverse function to swe_julday(), see the description there.
    /// Arguments are julian day number, calendar flag (0=julian, 1=gregorian)
    /// return values are the calendar day, month, year and the hour of
    /// the day with decimal fraction (0 .. 23.999999).
    /// Be aware the we use astronomical year numbering for the years
    /// before Christ, not the historical year numbering.
    /// Astronomical years are done with negative numbers, historical
    /// years with indicators BC or BCE (before common era).
    /// Year  0 (astronomical)  	= 1 BC historical year
    /// year -1 (astronomical) 	= 2 BC historical year
    /// year -234 (astronomical) 	= 235 BC historical year
    /// etc.
    pub fn swe_revjul(
        tjd: f64,
        gregflag: i32,
        year: *mut i32,
        month: *mut i32,
        day: *mut i32,
        hour: *mut f64,
    );

    /*
     * 14. House cups calculation
     */

    /// int swe_house_names(
    ///     int hsys);
    pub fn swe_house_name(hsys: c_int) -> *mut c_char;

    /// int swe_houses_ex(
    ///     double tjd_ut,
    ///     int32 iflag,
    ///     double geolat,
    ///     double geolon,
    ///     int hsys,
    ///     double *cusps,
    ///     double *ascmc);
    pub fn swe_houses_ex(
        tjd_ut: c_double,
        iflag: c_int,
        geolat: c_double,
        geolon: c_double,
        hsys: c_int,
        cusps: *mut c_double,
        ascmc: *mut c_double,
    ) -> c_int;

    /// int swe_houses_ex2(
    ///     double tjd_ut, int32 iflag, double geolat, double geolon, int hsys,
    ///     double *cusps, double *ascmc, double *cusp_speed, double *ascmc_speed, char *serr);
    pub fn swe_houses_ex2(
        tjd_ut: c_double,
        iflag: c_int,
        geolat: c_double,
        geolon: c_double,
        hsys: c_int,
        cusps: *mut c_double,
        ascmc: *mut c_double,
        cusp_speed: *mut c_double,
        ascmc_speed: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    /*
     * 17. Auxiliary functions
     */

    /// double swe_degnorm(double x);
    pub fn swe_degnorm(x: c_double) -> c_double;

    /// double swe_radnorm(double x);
    pub fn swe_radnorm(x: c_double) -> c_double;

    /// double swe_split_deg(
    ///     double ddeg,
    ///     int32 roundflag,
    ///     int32 *ideg,
    ///     int32 *imin,
    ///     int32 *isec,
    ///     double *dsecfr,
    ///     int32 *isgn);
    pub fn swe_split_deg(
        ddeg: c_double,
        roundflag: c_int,
        ideg: *mut c_int,
        imin: *mut c_int,
        isec: *mut c_int,
        cdegfr: *mut c_double,
        isgn: *mut c_int,
    ) -> c_double;

    /*
     * Ayanamsha Functions for sidereal mode
     */

    /// Sets the sidereal mode.
    ///
    /// # Parameters:
    /// - `sid_mode`: The sidereal mode as an integer.
    /// - `t0`: Reference epoch for ayanamsa.
    /// - `ayan_t0`: Ayanamsa value at reference epoch.
    ///
    /// C function: `void swe_set_sid_mode(int32 sid_mode, double t0, double ayan_t0);`
    pub fn swe_set_sid_mode(sid_mode: c_int, t0: c_double, ayan_t0: c_double);

    /// Allows to get ayanamsha name based on the integer number
    pub fn swe_get_ayanamsa_name(isidmode: i32) -> *const std::os::raw::c_char;

    /*
     * Function to calcualte planetary nodes
     */
    /// Compute planetary nodes and apsides (perihelia, aphelia, second focal points of
    /// the orbital eclipses).
    /// Both functions do exactly the same except that they expect a different time parameter.
    ///
    /// # Parameters:
    /// - `tjd_ut`: Julian day number in Universal Time
    /// - `ipl`: Planet Number
    /// - `iflag`: Calculation Flags
    /// - `method`: tells what kind of nodes or apsides are required:
    ///         1 -> Means nodes and apsides are calcualtated for the bodies that have them.
    ///              For the Moon and planets ME trhoguh NE, osculating ones for PL and the
    ///              asteroids. This is the default method, also used in method=0;
    ///         2 -> Osculating nodes and apsides are calculated for all bodies.
    ///         4 -> Osculating nodes and apsides are calculated for all bodies. With planets
    ///              beyond Jupiter, the nodes and apsides are calculated from barycentric
    ///              positions and speed. Cf. the explanations in swisseph.doc. If this bit is
    ///              combined with SE_NODBIT_MEAN, mean values are given for the planets Mercury - Neptune.
    ///         256-> The second focal point of the orbital ellipse is computed and returned in the array of
    ///               the aphelion. This bit can be combined with any other bit.
    /// - `xnasc`: array of 6 double for ascending node
    /// - `xndsc`: array of 6 double for descending node
    /// - `xperi`: array of 6 double for perihelion
    /// - `xaphe`: array of 6 double for aphelion
    /// - `serr` : character string to contain error messages, 256 chars
    /// P.S. When True Nodes needed, select (Osculating Nodes).
    pub fn swe_nod_aps_ut(
        tjd_ut: f64,
        ipl: i32,
        iflag: i32,
        method: i32,
        xnasc: *mut f64,
        xndsc: *mut f64,
        xperi: *mut f64,
        xaphe: *mut f64,
        serr: *mut c_char,
    ) -> i32;

    pub fn swe_fixstar_ut(
        star: *mut c_char,
        tjd_ut: c_double,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    pub fn swe_fixstar(
        star: *mut c_char,
        tjd_et: c_double,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    pub fn swe_fixstar2_ut(
        star: *mut c_char,
        tjd_ut: c_double,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    pub fn swe_fixstar2(
        star: *mut c_char,
        tjd_et: c_double,
        iflag: c_int,
        xx: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    pub fn swe_fixstar_mag(
        star: *mut c_char,
        mag: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

    pub fn swe_fixstar2_mag(
        star: *mut c_char,
        mag: *mut c_double,
        serr: *mut c_char,
    ) -> c_int;

}
