pub mod input;
pub mod state_machine;

#[tokio::main]
async fn main() {
    // Initialize inputs
    let inputs = input::Inputs::default();

    // Initialize state machine
    let mut state = state_machine::State::Idle;

    // Main loop at 20Hz
    let interval = tokio::time::Duration::from_secs_f32(1. / 20.);
    loop {
        // Wait for the next tick
        tokio::time::sleep(interval).await;

        // Update state machine with current inputs
        state.tick(&inputs);
    }
}
