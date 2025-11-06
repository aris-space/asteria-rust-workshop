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

Additionally `introduction` should contain some small examples to get used to some concepts and tools such as:
- `rustfmt`
- `clippy`
- `cargo test`
- `rustdoc` comments & how to see them

- `enums` & pattern matching
- `await`
- traits and derives
- `tracing`
- Ownership (maybe)

The very basic rust syntax is covered sufficiently on the web, such as this short video <https://youtu.be/br3GIIQeefY>.

