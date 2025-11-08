//! Simulates the rockets ascent and decent.

use protocols::api::{Location, Velocity};
use rand::Rng;

use crate::WICHLEN;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DeploymentStatus {
    None,
    Drogue,
    Main,
    BrokenChutes,
}

pub struct SimulationState {
    pub location: Location,
    pub velocity: Velocity,

    /// in kg
    fuel_remaining: f32,
    is_burning: bool,
    deployment_status: DeploymentStatus,
}

impl SimulationState {
    /// kg/s
    const MASS_FLOW: f32 = 1.0;

    /// Creates a new simulation state initialized at the launch location.
    pub fn new() -> Self {
        SimulationState {
            location: WICHLEN,
            velocity: Velocity::default(),
            fuel_remaining: 6.0,
            is_burning: false,
            deployment_status: DeploymentStatus::None,
        }
    }

    /// Advances the simulation by the given duration.
    pub fn tick(&mut self, dt: f32) {
        // Thrusting
        if self.is_burning {
            // Simple thrust model: constant acceleration while fuel remains
            let acceleration = 30.0; // m/s^2
            self.velocity.down += -acceleration * dt;
            self.velocity.north += 0.01 * acceleration * dt;
            self.velocity.east -= 0.03 * acceleration * dt;

            self.fuel_remaining -= Self::MASS_FLOW * dt;
            if self.fuel_remaining <= 0.0 {
                self.is_burning = false;
                self.fuel_remaining = 0.0;
            }

            if self.deployment_status != DeploymentStatus::None {
                eprintln!("Warning: parachute deployed during thrust phase");
                // Consider it a failure of the chutes, go ballistic from here on out
                self.is_burning = false;
                self.deployment_status = DeploymentStatus::BrokenChutes;
            }
        // Coasting
        } else {
            let gravity = 9.81; // m/s^2
            self.velocity.down += gravity * dt;
            // Simple drag model: constant drag factor based on deployment status
            let drag_rate = match self.deployment_status {
                DeploymentStatus::None | DeploymentStatus::BrokenChutes => 0.995,
                DeploymentStatus::Drogue => 0.96,
                DeploymentStatus::Main => 0.89,
            };
            self.velocity.down *= drag_rate;
            self.velocity.north *= drag_rate;
            self.velocity.east *= drag_rate;
        }

        // Integration
        self.location.altitude -= self.velocity.down * dt;
        self.location.latitude += f64::from(self.velocity.north * dt) / (40_007_863. / 360.); // Approx conversion m to degrees
        self.location.longitude += f64::from(self.velocity.east * dt)
            / ((40_075_017. / 360.) * self.location.latitude.to_radians().cos());

        // Collision handling
        if self.location.altitude <= WICHLEN.altitude {
            // Rocket has landed
            self.location.altitude = WICHLEN.altitude;
            self.velocity = Velocity::default();
        }
    }

    /// Causes the simulation to start the engine thrust.
    pub fn ignite_engine(&mut self) {
        if self.fuel_remaining > 0.0 {
            self.is_burning = true;
        }
    }

    /// Causes the simulation to simulate opening the drogue parachute.
    pub fn deploy_drogue(&mut self) {
        if self.deployment_status == DeploymentStatus::None {
            let mut rng = rand::thread_rng();
            let randomstuff: u32 = rng.random();
            if randomstuff % 2 == 0 {
                eprintln!("Drogue parachute deployment failed!");
                self.deployment_status = DeploymentStatus::BrokenChutes;
            } else {
                self.deployment_status = DeploymentStatus::Drogue;
            }
        } else {
            eprintln!("Drogue parachute already deployed");
        }
    }

    /// Causes the simulation to simulate opening the main parachute,
    /// if the drogue parachute has already been deployed.
    pub fn deploy_main(&mut self) {
        if self.deployment_status == DeploymentStatus::Drogue {
            self.deployment_status = DeploymentStatus::Main;
        } else {
            eprintln!("Cannot deploy main parachute before drogue");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;

    use protocols::api::Velocity;

    use crate::WICHLEN;

    #[test]
    fn ballistic() {
        let mut sim = super::SimulationState::new();
        let dt = 0.2;
        let mut time = 0.0;

        // Chill a bit on the ground
        while time < 10.0 {
            sim.tick(dt);
            assert_eq!(sim.location, WICHLEN, "moved before ignition");
            assert_eq!(sim.velocity, Velocity::default(), "moved before ignition");

            time += dt;
        }

        sim.ignite_engine();

        let mut apogee = f32::NAN;
        while time < 300.0 {
            sim.tick(dt);
            if sim.velocity.down > 0.0 {
                println!(
                    "Reached apogee at time {:.1}s, altitude {:.1}m",
                    time, sim.location.altitude
                );
                apogee = sim.location.altitude;
                assert!(!sim.is_burning, "still burning at apogee");
                break;
            }
            time += dt;
        }
        assert!(apogee > 3000., "Rocket did not reach apogee");

        let mut max_velocity: f32 = 0.0;

        while time < 300.0 {
            sim.tick(dt);
            max_velocity = max_velocity.max(sim.velocity.down.abs());
            if sim.location.altitude <= WICHLEN.altitude {
                println!(
                    "Landed at time {:.1}s, altitude {:.1}m, max descent velocity {:.1} m/s",
                    time, sim.location.altitude, max_velocity
                );
                break;
            }
            time += dt;
        }

        assert!(
            max_velocity > 100.,
            "Rocket decend was not ballistic enough"
        );
        assert!(
            (sim.location.altitude - super::WICHLEN.altitude).abs() < 1.0,
            "Rocket did not land correctly"
        );
    }

    #[test]
    fn chute_descend() {
        let mut sim = super::SimulationState::new();
        let dt = 0.2;
        let mut time = 0.0;
        let mut apogee_time = f32::NAN;

        sim.ignite_engine();

        while time < 300.0 {
            sim.tick(dt);
            if sim.velocity.down > 0.0 {
                println!(
                    "Reached apogee at time {:.1}s, altitude {:.1}m",
                    time, sim.location.altitude
                );
                apogee_time = time;
                assert!(!sim.is_burning, "still burning at apogee");
                break;
            }
            time += dt;
        }
        assert!(apogee_time.is_finite(), "Rocket did not reach apogee");

        sim.deploy_drogue();

        let mut max_velocity: f32 = 0.0;
        while time < 300.0 {
            sim.tick(dt);
            max_velocity = max_velocity.max(sim.velocity.down.abs());
            if sim.location.altitude <= WICHLEN.altitude + 500.0 {
                println!("deploying main, max drogue velocity {max_velocity:.1} m/s");
                break;
            }
            time += dt;
        }
        assert!(max_velocity < 100., "Rocket decend was too fast for drogue");
        assert!(max_velocity > 20., "Rocket decend was too slow for drogue");

        sim.deploy_main();

        let mut end_velocity: f32 = 0.0;
        while time < 300.0 {
            sim.tick(dt);
            if sim.velocity.down != 0. {
                end_velocity = sim.velocity.down.abs();
            }
            if sim.location.altitude <= WICHLEN.altitude {
                println!(
                    "Landed at time {:.1}s, altitude {:.1}m, end main velocity {:.1} m/s",
                    time, sim.location.altitude, end_velocity
                );
                break;
            }
            time += dt;
        }
        assert!(end_velocity < 20., "Rocket decend was too fast for main");
        assert!(end_velocity > 5., "Rocket decend was too slow for main");

        assert!(
            (sim.location.altitude - super::WICHLEN.altitude).abs() < 1.0,
            "Rocket did not land correctly"
        );
    }
}
