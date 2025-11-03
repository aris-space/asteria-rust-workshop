//! This module contains definitions of data types transferred between the components.

/// The data that the avionics can send to the flight computer.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SensorMessage {
    /// Current location
    LocationData(Location),
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
