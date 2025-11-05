//! This module contains definitions of data types transferred between the components.

/// The data that the avionics can send to the flight computer.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SensorMessage {
    /// Current location
    LocationData(Location),
    /// Current velocity
    VelocityData(Velocity),
}

impl SensorMessage {
    pub const COMMUNICATIONS_PORT: u16 = 4200;
}

/// Position in WGS84 inertial frame.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Location {
    /// Location Latitude WGS84 in `°`
    pub latitude: f64,
    /// Location Longitude WGS84 in `°`
    pub longitude: f64,
    /// Location Altitude above sealevel in `m`
    pub altitude: f32,
}

impl Location {
    pub const INVALID: Self = Location {
        latitude: f64::NAN,
        longitude: f64::NAN,
        altitude: f32::NAN,
    };
}

/// Velocity in NED frame.
#[derive(Debug, Default, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Velocity {
    /// Velocity North in `m/s`
    pub north: f32,
    /// Velocity East in `m/s`
    pub east: f32,
    /// Velocity Down in `m/s`
    pub down: f32,
}

impl Velocity {
    pub const INVALID: Self = Velocity {
        north: f32::NAN,
        east: f32::NAN,
        down: f32::NAN,
    };
}
