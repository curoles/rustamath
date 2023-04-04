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
    Pint,
    /// Cup
    Cup,
    /// Fluid ounce
    FluidOunce,
    /// Tablespoon
    Tablespoon,
    /// Teaspoon,
    Teaspoon,
    /// Canadian gallon
    CanadianGallon,
    /// UK gallon
    UkGallon,
    /// miles/h
    MilesPerHour,
    /// km/h
    KilometersPerHour,
    /// Knot
    Knot,
    /// kg
    Kilogram,
    /// Pound mass
    PoundMass,
    /// Ounce mass
    OunceMass,
    /// Ton
    Ton,
    /// Metric ton
    MetricTon,
    /// UK ton
    UkTon,
    /// Troy ounce
    TroyOunce,
    /// Carat
    Carat,
    /// Unified atomic mass
    UnifiedAtomicMass,
    /// Gram force
    GramForce,
    /// Pound force
    PoundForce,
    /// Kilopound force
    KilopoundForce,
    /// Poundal
    Poundal,
    /// Calorie
    Calorie,
    /// BTU
    Btu,
    /// Therm
    Therm,
    /// Horsepower
    Horsepower,
    /// Bar
    Bar,
    /// Standard atmosphere
    StdAtmosphere,
    /// Torr
    Torr,
    /// Meter of mercury
    MeterOfMercury,
    /// Inch of mercury
    InchOfMercury,
    /// Inch of water
    InchOfWater,
    /// PSI
    Psi,
    /// Poise
    Poise,
    /// Stokes
    Stokes,
    /// STILB
    Stilb,
    /// Lumen
    Lumen,
    /// Lux
    Lux,
    /// Phot
    Phot,
    /// Footcandle
    Footcandle,
    /// Lambert
    Lambert,
    /// Footlambert
    Footlambert,
    /// Curie
    Curie,
    /// Roentgen
    Roentgen,
    /// Rad
    Rad,
    /// Solar mass
    SolarMass,
    /// Bohr radius
    BohrRadius,
    /// Newton
    Newton,
    /// Dyne
    Dyne,
    /// Joule
    Joule,
    /// Erg
    Erg,
    /// STEFAN_BOLTZMANN_CONSTANT
    StefanBolzmannConstant,
    /// THOMSON_CROSS_SECTION
    ThomsonCrossSection,
    /// Bohr magneton
    BohrMagneton,
    /// Nuclear magneton
    NuclearMagneton,
    /// ELECTRON_MAGNETIC_MOMENT
    ElectronMagneticMoment,
    /// PROTON_MAGNETIC_MOMENT
    PhotonMagneticMoment,
    /// Faraday
    Faraday,
    /// Electron charge
    ElectronCharge,
    /// VACUUM_PERMITTIVITY
    VacuumPermittivity,
    /// VACUUM_PERMEABILITY
    VacuumPermeability,
    /// Debye
    Debye,
    /// Gauss
    Gauss,
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


