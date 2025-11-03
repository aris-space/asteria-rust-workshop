//! This module contains the input data for the flight computer

use protocols::api::Location;

/// Stores the current inputs of the system, given by the avionics
pub struct Inputs {
    pub location: Location,
}

impl Default for Inputs {
    fn default() -> Self {
        Inputs {
            location: Location::INVALID,
        }
    }
}
