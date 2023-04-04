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
pub const UNITS: [MksTuple; 20] = [
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
    (Name::MassNeutron,            MASS_NEUTRON_UNIT,             f64::MASS_NEUTRON,            "Mass of neutron"),
    (Name::Rydberg,                RYDBERG_UNIT,                  f64::RYDBERG,                 "Rydberg"),
    (Name::Boltzmann,              BOLTZMANN_UNIT,                f64::BOLTZMANN,               "Boltzmann"),
    (Name::MolarGas,               MOLAR_GAS_UNIT,                f64::MOLAR_GAS,               "Molar gas"),
    (Name::StandardGasVolume,      STANDARD_GAS_VOLUME_UNIT,      f64::STANDARD_GAS_VOLUME,     "Standard gas volume"),
    (Name::Second,                 SECOND_UNIT,                   f64::SECOND,                  "Second"),
    (Name::Minute,                 MINUTE_UNIT,                   f64::MINUTE,                  "Minute"),
    //(Name:: HOUR                 UNIT,              f64::,         ""),
    //(Name:: DAY,                 UNIT,              f64::,         ""),
    //(Name:: WEEK                 UNIT,              f64::,         ""),
    //(Name::Meter,                UNIT,              f64::,         ""),
    //(Name:: INCH,                UNIT,              f64::,         ""),
    //(Name:: FOOT                 UNIT,              f64::,         ""),
    //(Name:: YARD                 UNIT,              f64::,         ""),
    //(Name:: MILE,                UNIT,              f64::,         ""),
    //(Name:: NAUTICAL_MILE,       UNIT,              f64::,         ""),
    //(Name:: FATHOM,              UNIT,              f64::,         ""),
    //(Name:: MIL,                 UNIT,              f64::,         ""),
    //(Name:: POINT,               UNIT,              f64::,         ""),
    //(Name:: TEXPOINT,            UNIT,              f64::,         ""),
    //(Name:: MICRON,              UNIT,              f64::,         ""),
    //(Name:: ANGSTROM,            UNIT,              f64::,         ""),
    //(Name:: HECTARE,             UNIT,              f64::,         ""),
    //(Name:: ACRE,                UNIT,              f64::,         ""),
    //(Name:: BARN,                UNIT,              f64::,         ""),
    //(Name:: LITER,               UNIT,              f64::,         ""),
    //(Name:: US_GALLON,           UNIT,              f64::,         ""),
    //(Name:: QUART,               UNIT,              f64::,         ""),
    //(Name:: PINT,                UNIT,              f64::,         ""),
    //(Name:: CUP,                 UNIT,              f64::,         ""),
    //(Name:: FLUID_OUNCE,         UNIT,              f64::,         ""),
    //(Name:: TABLESPOON,          UNIT,              f64::,         ""),
    //(Name:: TEASPOON,            UNIT,              f64::,         ""),
    //(Name:: CANADIAN_GALLON,     UNIT,              f64::,         ""),
    //(Name:: UK_GALLON,           UNIT,              f64::,         ""),
    //(Name:: MILES_PER_HOUR,      UNIT,              f64::,         ""),
    (Name::KilometersPerHour, KILOMETERS_PER_HOUR_UNIT, f64::KILOMETERS_PER_HOUR, "km/h"),
    //(Name:: KNOT,                UNIT,              f64::,         ""),
    //(Name::Kilogram,             UNIT,              f64::,         ""),
    //(Name:: POUND_MASS,          UNIT,              f64::,         ""),
    //(Name:: OUNCE_MASS,          UNIT,              f64::,         ""),
    //(Name:: TON,                 UNIT,              f64::,         ""),
    //(Name:: METRIC_TON,          UNIT,              f64::,         ""),
    //(Name:: UK_TON,              UNIT,              f64::,         ""),
    //(Name:: TROY_OUNCE,          UNIT,              f64::,         ""),
    //(Name:: CARAT,               UNIT,              f64::,         ""),
    //(Name:: UNIFIED_ATOMIC_MASS, UNIT,              f64::,         ""),
    //(Name:: GRAM_FORCE,          UNIT,              f64::,         ""),
    //(Name:: POUND_FORCE,         UNIT,              f64::,         ""),
    //(Name:: KILOPOUND_FORCE,     UNIT,              f64::,         ""),
    //(Name:: POUNDAL,             UNIT,              f64::,         ""),
    //(Name:: CALORIE,             UNIT,              f64::,         ""),
    //(Name:: BTU,                 UNIT,              f64::,         ""),
    //(Name:: THERM,               UNIT,              f64::,         ""),
    //(Name:: HORSEPOWER,          UNIT,              f64::,         ""),
    //(Name:: BAR,                 UNIT,              f64::,         ""),
    //(Name:: STD_ATMOSPHERE,      UNIT,              f64::,         ""),
    //(Name:: TORR,                UNIT,              f64::,         ""),
    //(Name:: METER_OF_MERCURY,    UNIT,              f64::,         ""),
    //(Name:: INCH_OF_MERCURY,     UNIT,              f64::,         ""),
    //(Name:: INCH_OF_WATER,       UNIT,              f64::,         ""),
    //(Name:: PSI,       UNIT,              f64::,         ""),
    //(Name:: POISE,       UNIT,              f64::,         ""),
    //(Name:: STOKES,       UNIT,              f64::,         ""),
    //(Name:: STILB,       UNIT,              f64::,         ""),
    //(Name:: LUMEN,       UNIT,              f64::,         ""),
    //(Name:: LUX,       UNIT,              f64::,         ""),
    //(Name:: PHOT,       UNIT,              f64::,         ""),
    //(Name:: FOOTCANDLE,       UNIT,              f64::,         ""),
    //(Name:: LAMBERT,       UNIT,              f64::,         ""),
    //(Name:: FOOTLAMBERT,       UNIT,              f64::,         ""),
    //(Name:: CURIE,       UNIT,              f64::,         ""),
    //(Name:: ROENTGEN,       UNIT,              f64::,         ""),
    //(Name:: RAD,       UNIT,              f64::,         ""),
    //(Name:: SOLAR_MASS,       UNIT,              f64::,         ""),
    //(Name:: BOHR_RADIUS,       UNIT,              f64::,         ""),
    //(Name:: NEWTON,       UNIT,              f64::,         ""),
    //(Name:: DYNE,       UNIT,              f64::,         ""),
    //(Name:: JOULE,       UNIT,              f64::,         ""),
    //(Name:: ERG,       UNIT,              f64::,         ""),
    //(Name:: STEFAN_BOLTZMANN_CONSTANT,       UNIT,              f64::,         ""),
    //(Name:: THOMSON_CROSS_SECTION,       UNIT,              f64::,         ""),
    //(Name:: BOHR_MAGNETON,       UNIT,              f64::,         ""),
    //(Name:: NUCLEAR_MAGNETON,       UNIT,              f64::,         ""),
    //(Name:: ELECTRON_MAGNETIC_MOMENT,       UNIT,              f64::,         ""),
    //(Name:: PROTON_MAGNETIC_MOMENT,       UNIT,              f64::,         ""),
    //(Name:: FARADAY,       UNIT,              f64::,         ""),
    //(Name:: ELECTRON_CHARGE,       UNIT,              f64::,         ""),
    //(Name:: VACUUM_PERMITTIVITY,       UNIT,              f64::,         ""),
    //(Name:: VACUUM_PERMEABILITY,       UNIT,              f64::,         ""),
    //(Name:: DEBYE,       UNIT,              f64::,         ""),
    //(Name:: GAUSS,       UNIT,              f64::,         ""),
    ];


