# Tiny Flight Computer

This repository aims to contain a mock flight computer system for educational purposes.
There are 4 components to this system:
- `avionics-mock` contains a tiny simulation of a rocket with gives you sensor data and controls the engine.
- `flight-computer` then interacts with the avionics to take decisions.
- `mission-control` interacts with the flight computer to send commands and display telemetry.
- `protocols` contains serde definitions and helpers to communicate via JSON over HTTP between the other components.

Additionally `introduction` should contain some small examples to get used to some concepts and tools such as:
- `rustfmt`
- `clippy`
- `cargo test`
- `rustdoc` comments
- `enums`
- `traits` and derives
- Ownership (maybe)

The very basic rust syntax is covered sufficiently on the web, such as this short video <https://youtu.be/br3GIIQeefY>.
