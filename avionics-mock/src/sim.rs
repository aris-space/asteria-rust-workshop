//! Simulates the rockets ascent and decent.

use protocols::api::{Location, Velocity};

use crate::WICHLEN;

pub struct SimulationState {
    pub location: Location,
    pub velocity: Velocity,

    /// in kg
    fuel_remaining: f32,
    is_burning: bool,
}

impl SimulationState {
    /// kg/s
    const MASS_FLOW: f32 = 1.0;

    /// Creates a new simulation state initialized at the launch location.
    pub fn new() -> Self {
        SimulationState {
            location: WICHLEN,
            velocity: Velocity::default(),
            fuel_remaining: 12.0,
            is_burning: false,
        }
    }

    /// Advances the simulation by the given duration.
    pub fn tick(&mut self, dt: f32) {
        if self.is_burning {
            // Simple thrust model: constant acceleration while fuel remains
            let acceleration = 25.0; // m/s^2
            self.velocity.down += -acceleration * dt;
            self.fuel_remaining -= Self::MASS_FLOW * dt;
            if self.fuel_remaining <= 0.0 {
                self.is_burning = false;
                self.fuel_remaining = 0.0;
            }
        } else {
            // Simple gravity model: constant deceleration
            let gravity = 9.81; // m/s^2
            self.velocity.down += gravity * dt;
        }

        self.location.altitude -= self.velocity.down * dt;

        if self.location.altitude <= WICHLEN.altitude {
            // Rocket has landed
            self.location.altitude = WICHLEN.altitude;
            self.velocity.down = 0.0;
        }
    }

    pub fn ignite_engine(&mut self) {
        if self.fuel_remaining > 0.0 {
            self.is_burning = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use protocols::api::Velocity;

    use crate::WICHLEN;

    #[test]
    fn acends_and_descends() {
        let mut sim = super::SimulationState::new();
        let dt = 0.2;
        let mut time = 0.0;
        let mut apogee_time = f32::NAN;

        while time < 20.0 {
            sim.tick(dt);
            assert_eq!(sim.location, WICHLEN, "moved before ignition");
            assert_eq!(sim.velocity, Velocity::default(), "moved before ignition");

            time += dt;
        }

        sim.ignite_engine();

        while time < 300.0 {
            sim.tick(dt);
            if apogee_time.is_nan() && sim.velocity.down > 0.0 {
                println!(
                    "Reached apogee at time {:.1}s, altitude {:.1}m",
                    time, sim.location.altitude
                );
                apogee_time = time;
                assert!(!sim.is_burning, "still burning at apogee");
            }
            if apogee_time.is_finite() && sim.location.altitude <= WICHLEN.altitude {
                println!(
                    "Landed at time {:.1}s, altitude {:.1}m",
                    time, sim.location.altitude
                );
                break;
            }
            time += dt;
        }

        assert!(apogee_time.is_finite(), "Rocket did not reach apogee");
        assert!(
            (sim.location.altitude - super::WICHLEN.altitude).abs() < 1.0,
            "Rocket did not land correctly"
        );
    }
}
