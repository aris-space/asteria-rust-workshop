# Tiny Flight Computer

This repository contains a mock flight computer system for educational purposes, as well as additional exercise material.

## System overview
There are 4 components to this system:
- `avionics-mock` contains a tiny simulation of a rocket with gives you sensor data and controls the engine.
- `flight-computer` then interacts with the avionics to take decisions.
- `mission-control` interacts with the flight computer to send commands and display telemetry.
- `protocols` contains serde definitions and helpers to communicate via JSON over UDP between the other components.

The mission control cannot talk directly with the avionics,
and while the flight computer can display some logs,
those should be considered non-available during the flight.

## Additional material

I recommend first watching this short video <https://youtu.be/br3GIIQeefY>.

Afterwards, read through `introduction`.
This contains comments and small examples to get used to some concepts and tools such as:
- `rustfmt`
- `clippy`
- `cargo test`
- `rustdoc` comments & how to see them
- `enums` & pattern matching
- Ownership
- traits and derives
- `async` and `await`

## Agenda

1. 45' introduction with Q&A
2. 5-10min break
3. 20' introduction to the FC skeleton & build ignition command
4. 40' work session on exercises
5. 30' review session
6. putting things together

## Exercises

### Complete the State Machine

> Easy

The logic in the state machine for main deployment and touchdown is still missing.
Add it in a way you see fit.

### Simulate Random Chute Failure

> Easy

The separation mechanism so super reliable that we never witness failures so we have to simulate them to get the experience.
Add code to the avionics-mock simulator to, with some random chance,
not deploy the drogue chute when the command arrives.

### Mission Control - Telemetry

> Medium

Currently, we cannot see the data unless connected to the flight computer.
Write a mission control crate that listens to telemetry messages and displays the location and velocity.

### Mission Control - Commands

> Medium - Hard

Currently, the demo can only send the ignition command, and that also only in a tedious way.
You could make a nicer mission control crate that can listen to keyboard inputs.
You can also just make more binaries.

Also implementing manual separation and main deployment would be nice.

### Data Logging

> Medium - Hard

Currently the data is lost after the mission.
It would be nice to log it in CSV files.
Find a crate do help you with that and log the location and velocity to CSV files on the flight computer.

## TODO:
- [ ] Rand as google/llm exercise
- [ ] tracing
