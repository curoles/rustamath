//! List of MKS constants names
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!

use super::*;

/// List of MKS constants name
pub enum Name {
    /// Speef of light
    SpeedOfLight,
    /// Gravitational constant
    GravitationalConstant,
    /// Plancks h constant
    PlancksConstantH,
    /// Plancks h-bar constant
    PlancksConstantHBar,
    /// Astronomical unit
    AstronomicalUnit,
    /// Light year
    LightYear,
    /// Parsec
    Parsec,
    /// Acceleration
    GravAccel,
    /// Electron Volt
    ElectronVolt,
    /// Mass of electron
    MassElectron,
    /// Mass of muon
    MassMuon,
    /// Mass of proton
    MassProton,
    /// Mass of neutron
    MassNeutron,
    /// Rydberg
    Rydberg,
    /// Boltzmann
    Boltzmann,
    /// Molar
    MolarGas,
    /// Standard gas volume
    StandardGasVolume,
    /// Second of time
    Second,
    /// Minute of time
    Minute,
    /// Hour
    Hour,
    /// Day
    Day,
    /// Week
    Week,
    /// Meter
    Meter,
    /// Inch
    Inch,
    /// Foot
    Foot,
    /// Yard
    Yard,
    /// Mile
    Mile,
    /// NauticalMile
    NauticalMile,
    /// Fathom
    Fathom,
    /// Mil
    Mil,
    /// Point
    Point,
    /// Textpoint
    Textpoint,
    /// Micron
    Micron,
    /// Angstrom
    Angstrom,
    /// Hectare
    Hectare,
    /// Acre
    Acre,
    /// Barn
    Barn,
    /// Liter,
    Liter,
    /// US gallon
    UsGallon,
    /// Quart
    Quart,
    /// Pint
    Pint
    /// Cup
    Cup,

    //const FLUID_OUNCE: f64 = (2.95735295626e-5) /* m^3 */
    //const TABLESPOON: f64 = (1.47867647813e-5) /* m^3 */
    //const TEASPOON: f64 = (4.92892159375e-6) /* m^3 */
    //const CANADIAN_GALLON: f64 = (4.54609e-3) /* m^3 */
    //const UK_GALLON: f64 = (4.546092e-3) /* m^3 */
    //const MILES_PER_HOUR: f64 = (4.4704e-1) /* m / s */
    /// km/h
    KilometersPerHour,
    //const KNOT: f64 = (5.14444444444e-1) /* m / s */
    /// kg
    Kilogram,
    //const POUND_MASS: f64 = (4.5359237e-1) /* kg */
    //const OUNCE_MASS: f64 = (2.8349523125e-2) /* kg */
    //const TON: f64 = (9.0718474e2) /* kg */
    //const METRIC_TON: f64 = (1e3) /* kg */
    //const UK_TON: f64 = (1.0160469088e3) /* kg */
    //const TROY_OUNCE: f64 = (3.1103475e-2) /* kg */
    //const CARAT: f64 = (2e-4) /* kg */
    //const UNIFIED_ATOMIC_MASS: f64 = (1.660538782e-27) /* kg */
    //const GRAM_FORCE: f64 = (9.80665e-3) /* kg m / s^2 */
    //const POUND_FORCE: f64 = (4.44822161526e0) /* kg m / s^2 */
    //const KILOPOUND_FORCE: f64 = (4.44822161526e3) /* kg m / s^2 */
    //const POUNDAL: f64 = (1.38255e-1) /* kg m / s^2 */
    //const CALORIE: f64 = (4.1868e0) /* kg m^2 / s^2 */
    //const BTU: f64 = (1.05505585262e3) /* kg m^2 / s^2 */
    //const THERM: f64 = (1.05506e8) /* kg m^2 / s^2 */
    //const HORSEPOWER: f64 = (7.457e2) /* kg m^2 / s^3 */
    //const BAR: f64 = (1e5) /* kg / m s^2 */
    //const STD_ATMOSPHERE: f64 = (1.01325e5) /* kg / m s^2 */
    //const TORR: f64 = (1.33322368421e2) /* kg / m s^2 */
    //const METER_OF_MERCURY: f64 = (1.33322368421e5) /* kg / m s^2 */
    //const INCH_OF_MERCURY: f64 = (3.38638815789e3) /* kg / m s^2 */
    //const INCH_OF_WATER: f64 = (2.490889e2) /* kg / m s^2 */
    //const PSI (6.89475729317e3) /* kg / m s^2 */
    //const POISE (1e-1) /* kg m^-1 s^-1 */
    //const STOKES (1e-4) /* m^2 / s */
    //const STILB (1e4) /* cd / m^2 */
    //const LUMEN (1e0) /* cd sr */
    //const LUX (1e0) /* cd sr / m^2 */
    //const PHOT (1e4) /* cd sr / m^2 */
    //const FOOTCANDLE (1.076e1) /* cd sr / m^2 */
    //const LAMBERT (1e4) /* cd sr / m^2 */
    //const FOOTLAMBERT (1.07639104e1) /* cd sr / m^2 */
    //const CURIE (3.7e10) /* 1 / s */
    //const ROENTGEN (2.58e-4) /* A s / kg */
    //const RAD (1e-2) /* m^2 / s^2 */
    //const SOLAR_MASS (1.98892e30) /* kg */
    //const BOHR_RADIUS (5.291772083e-11) /* m */
    //const NEWTON (1e0) /* kg m / s^2 */
    //const DYNE (1e-5) /* kg m / s^2 */
    //const JOULE (1e0) /* kg m^2 / s^2 */
    //const ERG (1e-7) /* kg m^2 / s^2 */
    //const STEFAN_BOLTZMANN_CONSTANT (5.67040047374e-8) /* kg / K^4 s^3 */
    //const THOMSON_CROSS_SECTION (6.65245893699e-29) /* m^2 */
    //const BOHR_MAGNETON (9.27400899e-24) /* A m^2 */
    //const NUCLEAR_MAGNETON (5.05078317e-27) /* A m^2 */
    //const ELECTRON_MAGNETIC_MOMENT (9.28476362e-24) /* A m^2 */
    //const PROTON_MAGNETIC_MOMENT (1.410606633e-26) /* A m^2 */
    //const FARADAY (9.64853429775e4) /* A s / mol */
    //const ELECTRON_CHARGE (1.602176487e-19) /* A s */
    //const VACUUM_PERMITTIVITY (8.854187817e-12) /* A^2 s^4 / kg m^3 */
    //const VACUUM_PERMEABILITY (1.25663706144e-6) /* kg m / A^2 s^2 */
    //const DEBYE (3.33564095198e-30) /* A s^2 / m^2 */
    //const GAUSS (1e-4) /* kg / A s^2 */
}

/// Record in the list of constants
pub type MksTuple<'a> = (Name, MksUnit, f64, &'a str);

/// List of MKS units with dimentions and factors
pub const UNITS: [MksTuple; 13] = [
    (Name::SpeedOfLight,           SPEED_OF_LIGHT_UNIT,           f64::SPEED_OF_LIGHT,          "Speed of light"),
    (Name::GravitationalConstant,  GRAVITATIONAL_CONSTANT_UNIT,   f64::GRAVITATIONAL_CONSTANT,  "Gravitational constant"),
    (Name::PlancksConstantH,       PLANCKS_CONSTANT_H_UNIT,       f64::PLANCKS_CONSTANT_H,      "Planck's constant h"),
    (Name::PlancksConstantHBar,    PLANCKS_CONSTANT_HBAR_UNIT,    f64::PLANCKS_CONSTANT_HBAR,   "Planck's constant h bar"),
    (Name::AstronomicalUnit,       ASTRONOMICAL_UNIT_UNIT,        f64::ASTRONOMICAL_UNIT,       "Astronomical unit"),
    (Name::LightYear,              LIGHT_YEAR_UNIT,               f64::LIGHT_YEAR,              "Light year"),
    (Name::Parsec,                 PARSEC_UNIT,                   f64::PARSEC,                  "Parsec"),
    (Name::GravAccel,              GRAV_ACCEL_UNIT,               f64::GRAV_ACCEL,              "Grav Acceleration"),
    (Name::ElectronVolt,           ELECTRON_VOLT_UNIT,            f64::ELECTRON_VOLT,           "Electron Volt"),
    (Name::MassElectron,           MASS_ELECTRON_UNIT,            f64::MASS_ELECTRON,           "Mass of electron"),
    (Name::MassMuon,               MASS_MUON_UNIT,                f64::MASS_MUON,               "Mass of muon"),
    (Name::MassProton,             MASS_PROTON_UNIT,              f64::MASS_PROTON,             "Mass of proton"),
    //(Name:: MASS_NEUTRON
    //(Name:: RYDBERG
    //(Name:: BOLTZMANN
    //(Name:: MOLAR_GAS
    //(Name:: STANDARD_GAS_VOLUME
    //(Name::Second
    //(Name::Minute
    //(Name:: HOUR
    //(Name:: DAY
    //(Name:: WEEK
    //(Name::Meter
    //(Name:: INCH
    //(Name:: FOOT
    //(Name:: YARD
    //(Name:: MILE
    //(Name:: NAUTICAL_MILE
    //(Name:: FATHOM
    //(Name:: MIL
    //(Name:: POINT
    //(Name:: TEXPOINT
    //(Name:: MICRON
    //(Name:: ANGSTROM
    //(Name:: HECTARE
    //(Name:: ACRE
    //(Name:: BARN
    //(Name:: LITER
    //(Name:: US_GALLON
    //(Name:: QUART
    //(Name:: PINT
    //(Name:: CUP
    //(Name:: FLUID_OUNCE
    //(Name:: TABLESPOON
    //(Name:: TEASPOON
    //(Name:: CANADIAN_GALLON
    //(Name:: UK_GALLON
    //(Name:: MILES_PER_HOUR
    (Name::KilometersPerHour, KILOMETERS_PER_HOUR_UNIT, f64::KILOMETERS_PER_HOUR, "km/h"),
    //(Name:: KNOT
    //(Name::Kilogram
    //(Name:: POUND_MASS
    //(Name:: OUNCE_MASS
    //(Name:: TON
    //(Name:: METRIC_TON
    //(Name:: UK_TON
    //(Name:: TROY_OUNCE
    //(Name:: CARAT
    //(Name:: UNIFIED_ATOMIC_MASS
    //(Name:: GRAM_FORCE
    //(Name:: POUND_FORCE
    //(Name:: KILOPOUND_FORCE
    //(Name:: POUNDAL
    //(Name:: CALORIE
    //(Name:: BTU
    //(Name:: THERM
    //(Name:: HORSEPOWER
    //(Name:: BAR
    //(Name:: STD_ATMOSPHERE
    //(Name:: TORR
    //(Name:: METER_OF_MERCURY
    //(Name:: INCH_OF_MERCURY
    //(Name:: INCH_OF_WATER
    //(Name:: PSI
    //(Name:: POISE
    //(Name:: STOKES
    //(Name:: STILB
    //(Name:: LUMEN
    //(Name:: LUX
    //(Name:: PHOT
    //(Name:: FOOTCANDLE
    //(Name:: LAMBERT
    //(Name:: FOOTLAMBERT
    //(Name:: CURIE
    //(Name:: ROENTGEN
    //(Name:: RAD
    //(Name:: SOLAR_MASS
    //(Name:: BOHR_RADIUS
    //(Name:: NEWTON
    //(Name:: DYNE
    //(Name:: JOULE
    //(Name:: ERG
    //(Name:: STEFAN_BOLTZMANN_CONSTANT
    //(Name:: THOMSON_CROSS_SECTION
    //(Name:: BOHR_MAGNETON
    //(Name:: NUCLEAR_MAGNETON
    //(Name:: ELECTRON_MAGNETIC_MOMENT
    //(Name:: PROTON_MAGNETIC_MOMENT
    //(Name:: FARADAY
    //(Name:: ELECTRON_CHARGE
    //(Name:: VACUUM_PERMITTIVITY
    //(Name:: VACUUM_PERMEABILITY
    //(Name:: DEBYE
    //(Name:: GAUSS
    ];


