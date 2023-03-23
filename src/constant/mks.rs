//! MKS constants
//!
//! (c) Igor Lesik 2023
//! MIT license
//!
//! References:
//! - <https://github.com/ampl/gsl/blob/master/const/gsl_const_mks.h>

/// MKS unit as tuple of integer powers/dimentions (meter, kg, sec, ampere)
pub type MksUnit = (i32, i32, i32, i32);

/// Speed of light [m/s]
pub const SPEED_OF_LIGHT_UNIT:           MksUnit = ( 1,  0, -1,  0); // m / s
/// Gravitational constant
pub const GRAVITATIONAL_CONSTANT_UNIT:   MksUnit = ( 3, -1, -2,  0); // m^3 / kg s^2
/// Planks constant
pub const PLANCKS_CONSTANT_H_UNIT:       MksUnit = ( 2,  2, -1,  0); // kg m^2 / s
/// Planks bar constant
pub const PLANCKS_CONSTANT_HBAR_UNIT:    MksUnit = ( 2,  2, -1,  0); // kg m^2 / s
/// Astronomical unit of lenght
pub const ASTRONOMICAL_UNIT_UNIT:        MksUnit = ( 1,  0,  0,  0); // m
/// Light year
pub const LIGHT_YEAR_UNIT:               MksUnit = ( 1,  0,  0,  0); // m
/// Parsec
pub const PARSEC_UNIT:                   MksUnit = ( 1,  0,  0,  0); // m
/// Acceleration
pub const GRAV_ACCEL_UNIT:               MksUnit = ( 1,  0, -2,  0); // m / s^2
/// Electron Volt
pub const ELECTRON_VOLT_UNIT:            MksUnit = ( 2,  1, -2,  0); // kg m^2 / s^2
/// Mass of electron
pub const MASS_ELECTRON_UNIT:            MksUnit = ( 0,  1,  0,  0); // kg
/// Mass of muon
pub const MASS_MUON_UNIT:                MksUnit = ( 0,  1,  0,  0); // kg
/// Mass of proton
pub const MASS_PROTON_UNIT:              MksUnit = ( 0,  1,  0,  0); // kg
/// Mass neutron
pub const MASS_NEUTRON_UNIT:             MksUnit = ( 0,  1,  0,  0); // kg
/// Rydberg
pub const RYDBERG_UNIT:                  MksUnit = ( 2,  1, -2,  0); // kg m^2 / s^2
/// Boltzmann
pub const BOLTZMANN_UNIT:                MksUnit = ( 2,  1, -2,  0); // kg m^2 / K s^2
/// Molar of gas
pub const MOLAR_GAS_UNIT:                MksUnit = ( 2,  1, -2,  0); // kg m^2 / K mol s^2
/// Standard gas volume
pub const STANDARD_GAS_VOLUME_UNIT:      MksUnit = ( 3,  0,  0,  0); // m^3 / mol
/// One minute of time
pub const MINUTE_UNIT     :              MksUnit = ( 0,  0,  1,  0);
//pub const HOUR:                   MksUnit = ( 0,  0,  0,  0); /* s */
//pub const DAY:                    MksUnit = ( 0,  0,  0,  0); /* s */
//pub const WEEK:                   MksUnit = ( 0,  0,  0,  0); /* s */
//pub const INCH:                   MksUnit = ( 0,  0,  0,  0); /* m */
//pub const FOOT:                   MksUnit = ( 0,  0,  0,  0); /* m */
//pub const YARD:                   MksUnit = ( 0,  0,  0,  0); /* m */
//pub const MILE:                   MksUnit = ( 0,  0,  0,  0); /* m */
//pub const NAUTICAL_MILE:          MksUnit = ( 0,  0,  0,  0); /* m */
//pub const FATHOM:                 MksUnit = ( 0,  0,  0,  0); /* m */
//pub const MIL:                    MksUnit = ( 0,  0,  0,  0); /* m */
//pub const POINT:                  MksUnit = ( 0,  0,  0,  0);) /* m */
//pub const TEXPOINT:               MksUnit = ( 0,  0,  0,  0);) /* m */
//pub const MICRON:                 MksUnit = ( 0,  0,  0,  0); /* m */
//pub const ANGSTROM:               MksUnit = ( 0,  0,  0,  0); /* m */
//pub const HECTARE:                MksUnit = ( 0,  0,  0,  0); /* m^2 */
//pub const ACRE:                   MksUnit = ( 0,  0,  0,  0); /* m^2 */
//pub const BARN:                   MksUnit = ( 0,  0,  0,  0); /* m^2 */
//pub const LITER:                  MksUnit = ( 0,  0,  0,  0);/* m^3 */
//pub const US_GALLON:              MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const QUART:                  MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const PINT:                   MksUnit = ( 0,  0,  0,  0);) /* m^3 */
//pub const CUP:                    MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const FLUID_OUNCE:            MksUnit = ( 0,  0,  0,  0);) /* m^3 */
//pub const TABLESPOON:             MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const TEASPOON:               MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const CANADIAN_GALLON:        MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const UK_GALLON:              MksUnit = ( 0,  0,  0,  0); /* m^3 */
//pub const MILES_PER_HOUR:         MksUnit = ( 0,  0,  0,  0); /* m / s */
/// km/h dimentions is [m/s]
pub const KILOMETERS_PER_HOUR_UNIT: MksUnit = ( 1,  0, -1,  0);
//pub const KNOT: MksUnit = ( 0,  0,  0,  0); /* m / s */
//pub const POUND_MASS: MksUnit = ( 0,  0,  0,  0);/* kg */
//pub const OUNCE_MASS: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const TON: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const METRIC_TON: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const UK_TON: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const TROY_OUNCE: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const CARAT: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const UNIFIED_ATOMIC_MASS: MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const GRAM_FORCE: MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const POUND_FORCE: MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const KILOPOUND_FORCE: MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const POUNDAL: MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const CALORIE: MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const BTU: MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const THERM: MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const HORSEPOWER: MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^3 */
//pub const BAR: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const STD_ATMOSPHERE: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const TORR: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const METER_OF_MERCURY: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const INCH_OF_MERCURY: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const INCH_OF_WATER: MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const PSI MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const POISE MksUnit = ( 0,  0,  0,  0); /* kg m^-1 s^-1 */
//pub const STOKES MksUnit = ( 0,  0,  0,  0); /* m^2 / s */
//pub const STILB MksUnit = ( 0,  0,  0,  0); /* cd / m^2 */
//pub const LUMEN MksUnit = ( 0,  0,  0,  0);/* cd sr */
//pub const LUX MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const PHOT MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const FOOTCANDLE MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const LAMBERT MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const FOOTLAMBERT MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const CURIE MksUnit = ( 0,  0,  0,  0); /* 1 / s */
//pub const ROENTGEN MksUnit = ( 0,  0,  0,  0); /* A s / kg */
//pub const RAD MksUnit = ( 0,  0,  0,  0); /* m^2 / s^2 */
//pub const SOLAR_MASS MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const BOHR_RADIUS MksUnit = ( 0,  0,  0,  0); /* m */
//pub const NEWTON MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const DYNE MksUnit = ( 0,  0,  0,  0);/* kg m / s^2 */
//pub const JOULE MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const ERG MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const STEFAN_BOLTZMANN_CONSTANT MksUnit = ( 0,  0,  0,  0); /* kg / K^4 s^3 */
//pub const THOMSON_CROSS_SECTION MksUnit = ( 0,  0,  0,  0); /* m^2 */
//pub const BOHR_MAGNETON MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const NUCLEAR_MAGNETON MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const ELECTRON_MAGNETIC_MOMENT MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const PROTON_MAGNETIC_MOMENT MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const FARADAY MksUnit = ( 0,  0,  0,  0);/* A s / mol */
//pub const ELECTRON_CHARGE (MksUnit = ( 0,  0,  0,  0); /* A s */
//pub const VACUUM_PERMITTIVITY MksUnit = ( 0,  0,  0,  0); /* A^2 s^4 / kg m^3 */
//pub const VACUUM_PERMEABILITY MksUnit = ( 0,  0,  0,  0); /* kg m / A^2 s^2 */
//pub const DEBYE (MksUnit = ( 0,  0,  0,  0); /* A s^2 / m^2 */
//pub const GAUSS (MksUnit = ( 0,  0,  0,  0); /* kg / A s^2 */

/// Return true if units have the same dimentions
#[inline] pub fn is_unit_eq(a: MksUnit, b: MksUnit) -> bool {
    a == b
}

/// List of MKS units
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
    /// Minute of time
    Minute,
    //const HOUR:                   f64 = (3.6e3) /* s */
    //const DAY:                    f64 = (8.64e4) /* s */
    //const WEEK:                   f64 = (6.048e5) /* s */
    //const INCH:                   f64 = (2.54e-2) /* m */
    //const FOOT:                   f64 = (3.048e-1) /* m */
    //const YARD:                   f64 = (9.144e-1) /* m */
    //const MILE:                   f64 = (1.609344e3) /* m */
    //const NAUTICAL_MILE: f64 = (1.852e3) /* m */
    //const FATHOM: f64 = (1.8288e0) /* m */
    //const MIL: f64 = (2.54e-5) /* m */
    //const POINT: f64 = (3.52777777778e-4) /* m */
    //const TEXPOINT: f64 = (3.51459803515e-4) /* m */
    //const MICRON: f64 = (1e-6) /* m */
    //const ANGSTROM: f64 = (1e-10) /* m */
    //const HECTARE: f64 = (1e4) /* m^2 */
    //const ACRE: f64 = (4.04685642241e3) /* m^2 */
    //const BARN: f64 = (1e-28) /* m^2 */
    //const LITER: f64 = (1e-3) /* m^3 */
    //const US_GALLON: f64 = (3.78541178402e-3) /* m^3 */
    //const QUART: f64 = (9.46352946004e-4) /* m^3 */
    //const PINT: f64 = (4.73176473002e-4) /* m^3 */
    //const CUP: f64 = (2.36588236501e-4) /* m^3 */
    //const FLUID_OUNCE: f64 = (2.95735295626e-5) /* m^3 */
    //const TABLESPOON: f64 = (1.47867647813e-5) /* m^3 */
    //const TEASPOON: f64 = (4.92892159375e-6) /* m^3 */
    //const CANADIAN_GALLON: f64 = (4.54609e-3) /* m^3 */
    //const UK_GALLON: f64 = (4.546092e-3) /* m^3 */
    //const MILES_PER_HOUR: f64 = (4.4704e-1) /* m / s */
    /// km/h
    KilometersPerHour,
    //const KNOT: f64 = (5.14444444444e-1) /* m / s */
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

/// xxx
pub type MksTuple<'a> = (Name, MksUnit, f64, &'a str);

/// List of MKS units with dimentions and factors
pub const UNITS: [MksTuple; 8] = [
    (Name::SpeedOfLight,           SPEED_OF_LIGHT_UNIT,           f64::SPEED_OF_LIGHT,          "Speed of light"),
    (Name::GravitationalConstant,  GRAVITATIONAL_CONSTANT_UNIT,   f64::GRAVITATIONAL_CONSTANT,  "Gravitational constant"),
    (Name::PlancksConstantH,       PLANCKS_CONSTANT_H_UNIT,       f64::PLANCKS_CONSTANT_H,      "Planck's constant h"),
    (Name::PlancksConstantHBar,    PLANCKS_CONSTANT_HBAR_UNIT,    f64::PLANCKS_CONSTANT_HBAR,   "Planck's constant h bar"),
    (Name::AstronomicalUnit,       ASTRONOMICAL_UNIT_UNIT,        f64::ASTRONOMICAL_UNIT,       "Astronomical unit"),
    (Name::LightYear,              LIGHT_YEAR_UNIT,               f64::LIGHT_YEAR,              "Light year"),
    (Name::Parsec,                 PARSEC_UNIT,                   f64::PARSEC,                  "Parsec"),
    //const GRAV_ACCEL:             f64 = (9.80665e0) /* m / s^2 */
    //const ELECTRON_VOLT:          f64 = (1.602176487e-19) /* kg m^2 / s^2 */
    //const MASS_ELECTRON:          f64 = (9.10938188e-31) /* kg */
    //const MASS_MUON:              f64 = (1.88353109e-28) /* kg */
    //const MASS_PROTON:            f64 = (1.67262158e-27) /* kg */
    //const MASS_NEUTRON:           f64 = (1.67492716e-27) /* kg */
    //const RYDBERG:                f64 = (2.17987196968e-18) /* kg m^2 / s^2 */
    //const BOLTZMANN:              f64 = (1.3806504e-23) /* kg m^2 / K s^2 */
    //const MOLAR_GAS:              f64 = (8.314472e0) /* kg m^2 / K mol s^2 */
    //const STANDARD_GAS_VOLUME:    f64 = (2.2710981e-2) /* m^3 / mol */
    //Name::Minute
    //const HOUR:                   f64 = (3.6e3) /* s */
    //const DAY:                    f64 = (8.64e4) /* s */
    //const WEEK:                   f64 = (6.048e5) /* s */
    //const INCH:                   f64 = (2.54e-2) /* m */
    //const FOOT:                   f64 = (3.048e-1) /* m */
    //const YARD:                   f64 = (9.144e-1) /* m */
    //const MILE:                   f64 = (1.609344e3) /* m */
    //const NAUTICAL_MILE: f64 = (1.852e3) /* m */
    //const FATHOM: f64 = (1.8288e0) /* m */
    //const MIL: f64 = (2.54e-5) /* m */
    //const POINT: f64 = (3.52777777778e-4) /* m */
    //const TEXPOINT: f64 = (3.51459803515e-4) /* m */
    //const MICRON: f64 = (1e-6) /* m */
    //const ANGSTROM: f64 = (1e-10) /* m */
    //const HECTARE: f64 = (1e4) /* m^2 */
    //const ACRE: f64 = (4.04685642241e3) /* m^2 */
    //const BARN: f64 = (1e-28) /* m^2 */
    //const LITER: f64 = (1e-3) /* m^3 */
    //const US_GALLON: f64 = (3.78541178402e-3) /* m^3 */
    //const QUART: f64 = (9.46352946004e-4) /* m^3 */
    //const PINT: f64 = (4.73176473002e-4) /* m^3 */
    //const CUP: f64 = (2.36588236501e-4) /* m^3 */
    //const FLUID_OUNCE: f64 = (2.95735295626e-5) /* m^3 */
    //const TABLESPOON: f64 = (1.47867647813e-5) /* m^3 */
    //const TEASPOON: f64 = (4.92892159375e-6) /* m^3 */
    //const CANADIAN_GALLON: f64 = (4.54609e-3) /* m^3 */
    //const UK_GALLON: f64 = (4.546092e-3) /* m^3 */
    //const MILES_PER_HOUR: f64 = (4.4704e-1) /* m / s */
    (Name::KilometersPerHour, KILOMETERS_PER_HOUR_UNIT, f64::KILOMETERS_PER_HOUR, "km/h"),
    //const KNOT: f64 = (5.14444444444e-1) /* m / s */
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
];

/// Constant factors for MKS units
pub trait Mks
where
    Self: Copy,
    Self: core::ops::Mul<Output = Self>,
    Self: core::ops::Div<Output = Self>,
{

    /// Scale a number by Unit.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::mks::{Mks};
    /// assert_eq!(1.2_f64.to_units(f64::SPEED_OF_LIGHT), 2.99792458e8_f64 * 1.2)
    /// ```
    fn to_units(&self, unit: Self) -> Self {
        *self * unit
    }

    /// Divide a number by Unit.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::constant::mks::*;
    /// use assert_float_eq::*;
    /// assert!(is_unit_eq(SPEED_OF_LIGHT_UNIT, KILOMETERS_PER_HOUR_UNIT));
    /// assert!(SPEED_OF_LIGHT_UNIT != MINUTE_UNIT);
    /// assert_float_absolute_eq!(
    ///     1.0_f64.to_units(f64::SPEED_OF_LIGHT).in_units(f64::KILOMETERS_PER_HOUR),
    ///     1.079e9_f64, 1.0e6);
    /// ```
    fn in_units(&self, unit: Self) -> Self {
        *self / unit
    }

    /// Speed of light
    const SPEED_OF_LIGHT: Self;
    /// Gravitational constant
    const GRAVITATIONAL_CONSTANT: Self;
    /// Plank h constant
    const PLANCKS_CONSTANT_H: Self;
    /// Plank h-bar constant
    const PLANCKS_CONSTANT_HBAR: Self;
    /// Astronomical unit
    const ASTRONOMICAL_UNIT: Self;
    /// Light year
    const LIGHT_YEAR: Self;
    /// Parsec
    const PARSEC: Self;
    /// Acceleration
    const GRAV_ACCEL: Self;
    /// Electron Volt
    const ELECTRON_VOLT: Self;
    /// Mass of electron
    const MASS_ELECTRON: Self;
    /// Mass of muon
    const MASS_MUON: Self;
    /// Mass of proton
    const MASS_PROTON: Self;
    /// Mass of neutron
    const MASS_NEUTRON: Self;
    /// Rydberg
    const RYDBERG:Self;
    /// Boltzmann
    const BOLTZMANN: Self;
    /// Molar of gas
    const MOLAR_GAS: Self;
    /// Standard gas volume
    const STANDARD_GAS_VOLUME:Self;
    /// One minute of time, 60s
    const MINUTE: Self;
    //const HOUR:                   f64 = (3.6e3) /* s */
    //const DAY:                    f64 = (8.64e4) /* s */
    //const WEEK:                   f64 = (6.048e5) /* s */
    //const INCH:                   f64 = (2.54e-2) /* m */
    //const FOOT:                   f64 = (3.048e-1) /* m */
    //const YARD:                   f64 = (9.144e-1) /* m */
    //const MILE:                   f64 = (1.609344e3) /* m */
    //const NAUTICAL_MILE: f64 = (1.852e3) /* m */
    //const FATHOM: f64 = (1.8288e0) /* m */
    //const MIL: f64 = (2.54e-5) /* m */
    //const POINT: f64 = (3.52777777778e-4) /* m */
    //const TEXPOINT: f64 = (3.51459803515e-4) /* m */
    //const MICRON: f64 = (1e-6) /* m */
    //const ANGSTROM: f64 = (1e-10) /* m */
    //const HECTARE: f64 = (1e4) /* m^2 */
    //const ACRE: f64 = (4.04685642241e3) /* m^2 */
    //const BARN: f64 = (1e-28) /* m^2 */
    //const LITER: f64 = (1e-3) /* m^3 */
    //const US_GALLON: f64 = (3.78541178402e-3) /* m^3 */
    //const QUART: f64 = (9.46352946004e-4) /* m^3 */
    //const PINT: f64 = (4.73176473002e-4) /* m^3 */
    //const CUP: f64 = (2.36588236501e-4) /* m^3 */
    //const FLUID_OUNCE: f64 = (2.95735295626e-5) /* m^3 */
    //const TABLESPOON: f64 = (1.47867647813e-5) /* m^3 */
    //const TEASPOON: f64 = (4.92892159375e-6) /* m^3 */
    //const CANADIAN_GALLON: f64 = (4.54609e-3) /* m^3 */
    //const UK_GALLON: f64 = (4.546092e-3) /* m^3 */
    //const MILES_PER_HOUR: f64 = (4.4704e-1) /* m / s */
    /// km/h
    const KILOMETERS_PER_HOUR: Self;
    //const KNOT: f64 = (5.14444444444e-1) /* m / s */
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

impl Mks for f64 {
    const SPEED_OF_LIGHT:           f64 = 2.99792458e8_f64; // m / s
    const GRAVITATIONAL_CONSTANT:   f64 = 6.673e-11_f64; // m^3 / kg s^2
    const PLANCKS_CONSTANT_H:       f64 = 6.62606896e-34_f64; // kg m^2 / s
    const PLANCKS_CONSTANT_HBAR:    f64 = 1.05457162825e-34_f64; // kg m^2 / s
    const ASTRONOMICAL_UNIT:        f64 = 1.49597870691e11_f64; // m
    const LIGHT_YEAR:               f64 = 9.46053620707e15_f64; // m
    const PARSEC:                   f64 = 3.08567758135e16_f64; /* m */
    const GRAV_ACCEL:               f64 = 9.80665e0_f64; /* m / s^2 */
    const ELECTRON_VOLT:            f64 = 1.602176487e-19_f64; /* kg m^2 / s^2 */
    const MASS_ELECTRON:            f64 = 9.10938188e-31_f64; /* kg */
    const MASS_MUON:                f64 = 1.88353109e-28_f64; /* kg */
    const MASS_PROTON:              f64 = 1.67262158e-27_f64; /* kg */
    const MASS_NEUTRON:             f64 = 1.67492716e-27_f64; /* kg */
    const RYDBERG:                  f64 = 2.17987196968e-18_f64; /* kg m^2 / s^2 */
    const BOLTZMANN:                f64 = 1.3806504e-23_f64; /* kg m^2 / K s^2 */
    const MOLAR_GAS:                f64 = 8.314472e0_f64; /* kg m^2 / K mol s^2 */
    const STANDARD_GAS_VOLUME:      f64 = 2.2710981e-2_f64; /* m^3 / mol */
    const MINUTE:                   f64 = 6.0e1_f64; // s
    //const HOUR:                   f64 = (3.6e3) /* s */
    //const DAY:                    f64 = (8.64e4) /* s */
    //const WEEK:                   f64 = (6.048e5) /* s */
    //const INCH:                   f64 = (2.54e-2) /* m */
    //const FOOT:                   f64 = (3.048e-1) /* m */
    //const YARD:                   f64 = (9.144e-1) /* m */
    //const MILE:                   f64 = (1.609344e3) /* m */
    //const NAUTICAL_MILE: f64 = (1.852e3) /* m */
    //const FATHOM: f64 = (1.8288e0) /* m */
    //const MIL: f64 = (2.54e-5) /* m */
    //const POINT: f64 = (3.52777777778e-4) /* m */
    //const TEXPOINT: f64 = (3.51459803515e-4) /* m */
    //const MICRON: f64 = (1e-6) /* m */
    //const ANGSTROM: f64 = (1e-10) /* m */
    //const HECTARE: f64 = (1e4) /* m^2 */
    //const ACRE: f64 = (4.04685642241e3) /* m^2 */
    //const BARN: f64 = (1e-28) /* m^2 */
    //const LITER: f64 = (1e-3) /* m^3 */
    //const US_GALLON: f64 = (3.78541178402e-3) /* m^3 */
    //const QUART: f64 = (9.46352946004e-4) /* m^3 */
    //const PINT: f64 = (4.73176473002e-4) /* m^3 */
    //const CUP: f64 = (2.36588236501e-4) /* m^3 */
    //const FLUID_OUNCE: f64 = (2.95735295626e-5) /* m^3 */
    //const TABLESPOON: f64 = (1.47867647813e-5) /* m^3 */
    //const TEASPOON: f64 = (4.92892159375e-6) /* m^3 */
    //const CANADIAN_GALLON: f64 = (4.54609e-3) /* m^3 */
    //const UK_GALLON: f64 = (4.546092e-3) /* m^3 */
    //const MILES_PER_HOUR: f64 = (4.4704e-1) /* m / s */
    const KILOMETERS_PER_HOUR: f64 = 2.77777777778e-1; // m / s
    //const KNOT: f64 = (5.14444444444e-1) /* m / s */
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