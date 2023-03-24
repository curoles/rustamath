//! MKS constants
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//! - <https://github.com/ampl/gsl/blob/master/const/gsl_const_mks.h>
//!

pub mod list;

/// MKS unit as tuple of integer powers/dimentions (meter, kg, sec, ampere)
#[derive(Debug)]
pub struct MksUnit {
    m: i8, k: i8, s: i8, a: i8
}

impl std::cmp::PartialEq for MksUnit {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m && self.k == other.k && self.s == other.s && self.a == other.a
    }
}

impl std::ops::Mul for MksUnit {
    type Output = Self;

    /// Return unit type of multiplication of 2 values with unit type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::constant::mks::*;
    /// assert_eq!(SPEED_OF_LIGHT_UNIT * MINUTE_UNIT, LIGHT_YEAR_UNIT);
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Self {
            m: self.m + rhs.m,
            k: self.k + rhs.k,
            s: self.s + rhs.s,
            a: self.a + rhs.a
        }
    }
}

impl std::ops::Div for MksUnit {
    type Output = Self;

    /// Return unit type of division of 2 values with unit type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::constant::mks::*;
    /// assert_eq!(LIGHT_YEAR_UNIT / SPEED_OF_LIGHT_UNIT, MINUTE_UNIT);
    /// ```
    fn div(self, rhs: Self) -> Self {
        Self {
            m: self.m - rhs.m,
            k: self.k - rhs.k,
            s: self.s - rhs.s,
            a: self.a - rhs.a
        }
    }
}

/// Speed of light [m/s]
pub const SPEED_OF_LIGHT_UNIT:         MksUnit = MksUnit {m:  1, k:  0, s: -1, a: 0}; // m / s
/// Gravitational constant
pub const GRAVITATIONAL_CONSTANT_UNIT: MksUnit = MksUnit {m:  3, k: -1, s: -2, a: 0}; // m^3 / kg s^2
/// Planks constant
pub const PLANCKS_CONSTANT_H_UNIT:     MksUnit = MksUnit {m:  2, k:  2, s: -1, a: 0}; // kg m^2 / s
/// Planks bar constant
pub const PLANCKS_CONSTANT_HBAR_UNIT:  MksUnit = MksUnit {m:  2, k:  2, s: -1, a: 0}; // kg m^2 / s
/// Astronomical unit of lenght
pub const ASTRONOMICAL_UNIT_UNIT:      MksUnit = MksUnit {m:  1, k:  0, s:  0, a: 0}; // m
/// Light year
pub const LIGHT_YEAR_UNIT:             MksUnit = MksUnit {m:  1, k:  0, s:  0, a: 0}; // m
/// Parsec
pub const PARSEC_UNIT:                 MksUnit = MksUnit {m:  1, k:  0, s:  0, a: 0}; // m
/// Acceleration
pub const GRAV_ACCEL_UNIT:             MksUnit = MksUnit {m:  1, k:  0, s: -2, a: 0}; // m / s^2
/// Electron Volt
pub const ELECTRON_VOLT_UNIT:          MksUnit = MksUnit {m:  2, k:  1, s: -2, a: 0}; // kg m^2 / s^2
/// Mass of electron
pub const MASS_ELECTRON_UNIT:          MksUnit = MksUnit {m:  0, k:  1, s:  0, a: 0}; // kg
/// Mass of muon
pub const MASS_MUON_UNIT:              MksUnit = MksUnit {m:  0, k:  1, s:  0, a: 0}; // kg
/// Mass of proton
pub const MASS_PROTON_UNIT:            MksUnit = MksUnit {m:  0, k:  1, s:  0, a: 0}; // kg
/// Mass neutron
pub const MASS_NEUTRON_UNIT:           MksUnit = MksUnit {m:  0, k:  1, s:  0, a: 0}; // kg
/// Rydberg
pub const RYDBERG_UNIT:                MksUnit = MksUnit {m:  2, k:  1, s: -2, a: 0}; // kg m^2 / s^2
/// Boltzmann
pub const BOLTZMANN_UNIT:              MksUnit = MksUnit {m:  2, k:  1, s: -2, a: 0}; // kg m^2 / K s^2
/// Molar of gas
pub const MOLAR_GAS_UNIT:              MksUnit = MksUnit {m:  2, k:  1, s: -2, a: 0}; // kg m^2 / K mol s^2
/// Standard gas volume
pub const STANDARD_GAS_VOLUME_UNIT:    MksUnit = MksUnit {m:  3, k:  0, s:  0, a: 0}; // m^3 / mol
/// One second of time
pub const SECOND_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  1, a: 0}; // s
/// One minute of time
pub const MINUTE_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  1, a: 0}; // s
//pub const HOUR_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* s */
//pub const DAY_UNIT:                    MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* s */
//pub const WEEK_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* s */
//pub const METER_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const INCH_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const FOOT_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const YARD_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const MILE_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const NAUTICAL_MILE_UNIT:          MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const FATHOM_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}); /* m */
//pub const MIL_UNIT:                    MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const POINT_UNIT:                  MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0};) /* m */
//pub const TEXPOINT_UNIT:               MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0};) /* m */
//pub const MICRON_UNIT:                 MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const ANGSTROM_UNIT:               MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m */
//pub const HECTARE_UNIT:                MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^2 */
//pub const ACRE_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^2 */
//pub const BARN_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^2 */
//pub const LITER_UNIT:                  MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0};/* m^3 */
//pub const US_GALLON_UNIT:              MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const QUART_UNIT:                  MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const PINT_UNIT:                   MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0};) /* m^3 */
//pub const CUP_UNIT:                    MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const FLUID_OUNCE_UNIT:            MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0};) /* m^3 */
//pub const TABLESPOON_UNIT:             MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const TEASPOON_UNIT:               MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const CANADIAN_GALLON_UNIT:        MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const UK_GALLON_UNIT:              MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m^3 */
//pub const MILES_PER_HOUR_UNIT:         MksUnit = MksUnit {m:  0, k:  0, s:  0, a: 0}; /* m / s */
/// km/h dimentions is [m/s]
pub const KILOMETERS_PER_HOUR_UNIT: MksUnit = MksUnit {m: 1, k: 0, s: -1, a: 0};
//pub const KNOT:                        MksUnit = ( 0,  0,  0,  0); /* m / s */
//pub const KILOGRAM:                    MksUnit = ( 0,  0,  0,  0);/* kg */
//pub const POUND_MASS:                  MksUnit = ( 0,  0,  0,  0);/* kg */
//pub const OUNCE_MASS:                  MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const TON:                         MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const METRIC_TON:                  MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const UK_TON:                      MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const TROY_OUNCE:                  MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const CARAT:                       MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const UNIFIED_ATOMIC_MASS:         MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const GRAM_FORCE:                  MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const POUND_FORCE:                 MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const KILOPOUND_FORCE:             MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const POUNDAL:                     MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const CALORIE:                     MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const BTU:                         MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const THERM:                       MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const HORSEPOWER:                  MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^3 */
//pub const BAR:                         MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const STD_ATMOSPHERE:              MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const TORR:                        MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const METER_OF_MERCURY:            MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const INCH_OF_MERCURY:             MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const INCH_OF_WATER:               MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const PSI:                         MksUnit = ( 0,  0,  0,  0); /* kg / m s^2 */
//pub const POISE:                       MksUnit = ( 0,  0,  0,  0); /* kg m^-1 s^-1 */
//pub const STOKES:                      MksUnit = ( 0,  0,  0,  0); /* m^2 / s */
//pub const STILB:                       MksUnit = ( 0,  0,  0,  0); /* cd / m^2 */
//pub const LUMEN:                       MksUnit = ( 0,  0,  0,  0);/* cd sr */
//pub const LUX:                         MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const PHOT:                        MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const FOOTCANDLE:                  MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const LAMBERT:                     MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const FOOTLAMBERT:                 MksUnit = ( 0,  0,  0,  0); /* cd sr / m^2 */
//pub const CURIE:                       MksUnit = ( 0,  0,  0,  0); /* 1 / s */
//pub const ROENTGEN:                    MksUnit = ( 0,  0,  0,  0); /* A s / kg */
//pub const RAD:                         MksUnit = ( 0,  0,  0,  0); /* m^2 / s^2 */
//pub const SOLAR_MASS:                  MksUnit = ( 0,  0,  0,  0); /* kg */
//pub const BOHR_RADIUS:                 MksUnit = ( 0,  0,  0,  0); /* m */
//pub const NEWTON:                      MksUnit = ( 0,  0,  0,  0); /* kg m / s^2 */
//pub const DYNE:                        MksUnit = ( 0,  0,  0,  0);/* kg m / s^2 */
//pub const JOULE:                       MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const ERG:                         MksUnit = ( 0,  0,  0,  0); /* kg m^2 / s^2 */
//pub const STEFAN_BOLTZMANN_CONSTANT:   MksUnit = ( 0,  0,  0,  0); /* kg / K^4 s^3 */
//pub const THOMSON_CROSS_SECTION:       MksUnit = ( 0,  0,  0,  0); /* m^2 */
//pub const BOHR_MAGNETON:               MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const NUCLEAR_MAGNETON:            MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const ELECTRON_MAGNETIC_MOMENT:    MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const PROTON_MAGNETIC_MOMENT:      MksUnit = ( 0,  0,  0,  0); /* A m^2 */
//pub const FARADAY:                     MksUnit = ( 0,  0,  0,  0);/* A s / mol */
//pub const ELECTRON_CHARGE:             MksUnit = ( 0,  0,  0,  0); /* A s */
//pub const VACUUM_PERMITTIVITY:         MksUnit = ( 0,  0,  0,  0); /* A^2 s^4 / kg m^3 */
//pub const VACUUM_PERMEABILITY:         MksUnit = ( 0,  0,  0,  0); /* kg m / A^2 s^2 */
//pub const DEBYE:                       MksUnit = ( 0,  0,  0,  0); /* A s^2 / m^2 */
//pub const GAUSS:                       MksUnit = ( 0,  0,  0,  0); /* kg / A s^2 */


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
    /// assert!(SPEED_OF_LIGHT_UNIT == KILOMETERS_PER_HOUR_UNIT);
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
    /// One second of time
    const SECOND: Self;
    /// One minute of time, 60s
    const MINUTE: Self;
    /// Hour
    const HOUR: Self;
    /// Day
    const DAY: Self;
    /// Week
    const WEEK: Self;
    /// Meter
    const METER: Self;
    /// Inch
    const INCH: Self;
    /// Foot
    const FOOT: Self;
    /// Yard
    const YARD: Self;
    /// Mile
    const MILE:Self;
    /// Nautical mile
    const NAUTICAL_MILE: Self;
    /// Fathom
    const FATHOM: Self;
    /// Mil
    const MIL: Self;
    /// Point
    const POINT: Self;
    /// Textpoint
    const TEXPOINT: Self;
    /// Micron
    const MICRON: Self;
    /// Angstrom
    const ANGSTROM: Self;
    /// Hectare
    const HECTARE: Self;
    /// Acre
    const ACRE: Self;
    /// Barn
    const BARN: Self;
    /// Liter
    const LITER: Self;
    /// US gallon
    const US_GALLON: Self;
    /// Quart
    const QUART: Self;
    /// Pint
    const PINT: Self;
    /// Cup
    const CUP: Self;
    /// Fluid ounce
    const FLUID_OUNCE: Self;
    /// Tablespoon
    const TABLESPOON: Self;
    /// Teaspoon
    const TEASPOON: Self;
    /// Canadian gallon
    const CANADIAN_GALLON: Self;
    /// UK gallon
    const UK_GALLON: Self;
    /// miles/h
    const MILES_PER_HOUR: Self;
    /// km/h
    const KILOMETERS_PER_HOUR: Self;
    /// Knot
    const KNOT: Self;
    /// Kilogram
    const KILOGRAM: Self;
    /// Pound
    const POUND_MASS: Self;
    /// Ounce
    const OUNCE_MASS: Self;
    /// Ton
    const TON: Self;
    /// Metric ton
    const METRIC_TON: Self;
    /// UK ton
    const UK_TON: Self;
    /// Troy ounce
    const TROY_OUNCE: Self;
    /// Carat
    const CARAT: Self;
    /// Unified atomic mass
    const UNIFIED_ATOMIC_MASS: Self;
    /// Gram force
    const GRAM_FORCE: Self;
    /// Pound force
    const POUND_FORCE: Self;
    /// Kilopound force
    const KILOPOUND_FORCE: Self;
    /// Poundal
    const POUNDAL: Self;
    /// Calorie
    const CALORIE: Self;
    /// BTU
    const BTU: Self;
    /// Therm
    const THERM: Self;
    /// Horsepower
    const HORSEPOWER: Self;
    /// Bar
    const BAR: Self;
    /// STD atmosphere
    const STD_ATMOSPHERE: Self;
    /// Torr
    const TORR: Self;
    /// Meter of mercury
    const METER_OF_MERCURY: Self;
    /// Inch of mercury
    const INCH_OF_MERCURY: Self;
    /// Inch of water
    const INCH_OF_WATER: Self;
    /// PSI
    const PSI: Self;
    /// Poise
    const POISE: Self;
    /// Stokes
    const STOKES: Self;
    /// Stilb
    const STILB: Self;
    /// Lumen
    const LUMEN: Self;
    /// Lux
    const LUX: Self;
    /// Phot
    const PHOT: Self;
    /// Footcandle
    const FOOTCANDLE: Self;
    /// Lambert
    const LAMBERT: Self;
    /// Footlambert
    const FOOTLAMBERT: Self;
    /// Curie
    const CURIE: Self;
    /// Roentgen
    const ROENTGEN: Self;
    /// Rad
    const RAD: Self;
    /// Solar mass
    const SOLAR_MASS: Self;
    /// Bohr radius
    const BOHR_RADIUS: Self;
    /// Newton
    const NEWTON: Self;
    /// Dyne
    const DYNE: Self;
    /// Joule
    const JOULE: Self;
    /// Erg
    const ERG: Self;
    /// STEFAN_BOLTZMANN_CONSTANT
    const STEFAN_BOLTZMANN_CONSTANT: Self;
    /// THOMSON_CROSS_SECTION
    const THOMSON_CROSS_SECTION: Self;
    /// Bohr magneton
    const BOHR_MAGNETON: Self;
    /// Nuclear magneton
    const NUCLEAR_MAGNETON: Self;
    /// Electron magnetic moment
    const ELECTRON_MAGNETIC_MOMENT: Self;
    /// Proton magnetic moment
    const PROTON_MAGNETIC_MOMENT: Self;
    /// Faraday
    const FARADAY: Self;
    /// Electron charge
    const ELECTRON_CHARGE: Self;
    /// VACUUM_PERMITTIVITY
    const VACUUM_PERMITTIVITY: Self;
    /// VACUUM_PERMEABILITY
    const VACUUM_PERMEABILITY: Self;
    /// Debye
    const DEBYE: Self;
    /// Gauss
    const GAUSS: Self;
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
    const SECOND:                   f64 = 1.0_f64; // s
    const MINUTE:                   f64 = 6.0e1_f64; // s
    const HOUR:                     f64 = 3.6e3_f64; // s
    const DAY:                      f64 = 8.64e4_f64; // s
    const WEEK:                     f64 = 6.048e5_f64; // s
    const METER:                    f64 = 1.0_f64; // m
    const INCH:                     f64 = 2.54e-2_f64; /* m */
    const FOOT:                     f64 = 3.048e-1_f64; /* m */
    const YARD:                     f64 = 9.144e-1_f64; /* m */
    const MILE:                     f64 = 1.609344e3_f64; /* m */
    const NAUTICAL_MILE:            f64 = 1.852e3_f64; /* m */
    const FATHOM:                   f64 = 1.8288e0_f64; /* m */
    const MIL:                      f64 = 2.54e-5_f64; /* m */
    const POINT:                    f64 = 3.52777777778e-4_f64; /* m */
    const TEXPOINT:                 f64 = 3.51459803515e-4_f64; /* m */
    const MICRON:                   f64 = 1e-6_f64; /* m */
    const ANGSTROM:                 f64 = 1e-10_f64; /* m */
    const HECTARE:                  f64 = 1e4_f64; /* m^2 */
    const ACRE:                     f64 = 4.04685642241e3_f64; /* m^2 */
    const BARN:                     f64 = 1e-28_f64; /* m^2 */
    const LITER:                    f64 = 1e-3_f64; /* m^3 */
    const US_GALLON:                f64 = 3.78541178402e-3_f64; /* m^3 */
    const QUART:                    f64 = 9.46352946004e-4_f64; /* m^3 */
    const PINT:                     f64 = 4.73176473002e-4_f64; /* m^3 */
    const CUP:                      f64 = 2.36588236501e-4_f64; /* m^3 */
    const FLUID_OUNCE:              f64 = 2.95735295626e-5_f64; /* m^3 */
    const TABLESPOON:               f64 = 1.47867647813e-5_f64; /* m^3 */
    const TEASPOON:                 f64 = 4.92892159375e-6_f64; /* m^3 */
    const CANADIAN_GALLON:          f64 = 4.54609e-3_f64; /* m^3 */
    const UK_GALLON:                f64 = 4.546092e-3_f64; /* m^3 */
    const MILES_PER_HOUR:           f64 = 4.4704e-1_f64; /* m / s */
    const KILOMETERS_PER_HOUR:      f64 = 2.77777777778e-1_f64; // m / s
    const KNOT:                     f64 = 5.14444444444e-1_f64; /* m / s */
    const KILOGRAM:                 f64 = 1.0_f64; // kg
    const POUND_MASS:               f64 = 4.5359237e-1_f64; /* kg */
    const OUNCE_MASS:               f64 = 2.8349523125e-2_f64; /* kg */
    const TON:                      f64 = 9.0718474e2_f64; /* kg */
    const METRIC_TON:               f64 = 1e3_f64; /* kg */
    const UK_TON:                   f64 = 1.0160469088e3_f64; /* kg */
    const TROY_OUNCE:               f64 = 3.1103475e-2_f64; /* kg */
    const CARAT:                    f64 = 2e-4_f64; /* kg */
    const UNIFIED_ATOMIC_MASS:      f64 = 1.660538782e-27_f64; /* kg */
    const GRAM_FORCE:               f64 = 9.80665e-3_f64; /* kg m / s^2 */
    const POUND_FORCE:              f64 = 4.44822161526e0_f64; /* kg m / s^2 */
    const KILOPOUND_FORCE:          f64 = 4.44822161526e3_f64; /* kg m / s^2 */
    const POUNDAL:                  f64 = 1.38255e-1_f64; /* kg m / s^2 */
    const CALORIE:                  f64 = 4.1868e0_f64; /* kg m^2 / s^2 */
    const BTU:                      f64 = 1.05505585262e3_f64; /* kg m^2 / s^2 */
    const THERM:                    f64 = 1.05506e8_f64; /* kg m^2 / s^2 */
    const HORSEPOWER:               f64 = 7.457e2_f64; /* kg m^2 / s^3 */
    const BAR:                      f64 = 1e5_f64; /* kg / m s^2 */
    const STD_ATMOSPHERE:           f64 = 1.01325e5_f64; /* kg / m s^2 */
    const TORR:                     f64 = 1.33322368421e2_f64; /* kg / m s^2 */
    const METER_OF_MERCURY:         f64 = 1.33322368421e5_f64; /* kg / m s^2 */
    const INCH_OF_MERCURY:          f64 = 3.38638815789e3_f64; /* kg / m s^2 */
    const INCH_OF_WATER:            f64 = 2.490889e2_f64; /* kg / m s^2 */
    const PSI:                      f64 = 6.89475729317e3_f64; /* kg / m s^2 */
    const POISE:                    f64 = 1e-1_f64; /* kg m^-1 s^-1 */
    const STOKES:                   f64 = 1e-4_f64; /* m^2 / s */
    const STILB:                    f64 = 1e4_f64; /* cd / m^2 */
    const LUMEN:                    f64 = 1e0_f64; /* cd sr */
    const LUX:                      f64 = 1e0_f64; /* cd sr / m^2 */
    const PHOT:                     f64 = 1e4_f64; /* cd sr / m^2 */
    const FOOTCANDLE:               f64 = 1.076e1_f64; /* cd sr / m^2 */
    const LAMBERT:                  f64 = 1e4_f64; /* cd sr / m^2 */
    const FOOTLAMBERT:              f64 = 1.07639104e1_f64; /* cd sr / m^2 */
    const CURIE:                    f64 = 3.7e10_f64; /* 1 / s */
    const ROENTGEN:                 f64 = 2.58e-4_f64; /* A s / kg */
    const RAD:                      f64 = 1e-2_f64; /* m^2 / s^2 */
    const SOLAR_MASS:               f64 = 1.98892e30_f64; /* kg */
    const BOHR_RADIUS:              f64 = 5.291772083e-11_f64; /* m */
    const NEWTON:                   f64 = 1e0_f64; /* kg m / s^2 */
    const DYNE:                     f64 = 1e-5_f64; /* kg m / s^2 */
    const JOULE:                    f64 = 1e0_f64; /* kg m^2 / s^2 */
    const ERG:                      f64 = 1e-7_f64; /* kg m^2 / s^2 */
    const STEFAN_BOLTZMANN_CONSTANT:f64 = 5.67040047374e-8_f64; /* kg / K^4 s^3 */
    const THOMSON_CROSS_SECTION:    f64 = 6.65245893699e-29_f64; /* m^2 */
    const BOHR_MAGNETON:            f64 = 9.27400899e-24_f64; /* A m^2 */
    const NUCLEAR_MAGNETON:         f64 = 5.05078317e-27_f64; /* A m^2 */
    const ELECTRON_MAGNETIC_MOMENT: f64 = 9.28476362e-24_f64; /* A m^2 */
    const PROTON_MAGNETIC_MOMENT:   f64 = 1.410606633e-26_f64; /* A m^2 */
    const FARADAY:                  f64 = 9.64853429775e4_f64; /* A s / mol */
    const ELECTRON_CHARGE:          f64 = 1.602176487e-19_f64; /* A s */
    const VACUUM_PERMITTIVITY:      f64 = 8.854187817e-12_f64; /* A^2 s^4 / kg m^3 */
    const VACUUM_PERMEABILITY:      f64 = 1.25663706144e-6_f64; /* kg m / A^2 s^2 */
    const DEBYE:                    f64 = 3.33564095198e-30_f64; /* A s^2 / m^2 */
    const GAUSS:                    f64 = 1e-4_f64; /* kg / A s^2 */
}