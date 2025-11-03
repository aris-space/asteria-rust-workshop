//! This module contains definitions of data types transferred between the components.

/// The data that the avionics can send to the flight computer.
pub enum SensorMessage {
    /// Current location
    LocationData(Location),
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
