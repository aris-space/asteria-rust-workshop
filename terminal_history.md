# Terminal history
This file records the most important commands that I ran in the command line

## Technique
I just used a weird prompt function:

```fish
function fish_prompt
    # Save command to log file, relative to the hard coded location on my drive
    # eg. - `.>` `echo Hi`
    echo "- `"(string replace -r '^/Users/cyrill/Documents/Projects/2025 Asteria/rust-workshop' '.' (pwd))">` `"(history --max 1)"`" >> "/Users/cyrill/Documents/Projects/2025 Asteria/rust-workshop/terminal_history.md"
    
    # Still print the prompt
    echo -n (set_color green)(prompt_pwd)(set_color normal)
    echo -n "> "
end
```

## History (fish)
- `.>` `cargo new --lib protocols` Creates the `protocols library.
- `./protocols>` `cargo add serde --features derive`
- `./protocols>` `cargo add serde_json` Add a dependency to serialize using json
- `./protocols>` `cargo add tokio --features net` Add the runtime for async network communication
- `./protocols>` `cargo add tokio --features io-util` Add another feature to an existing dependency
- `./protocols>` `cargo add tokio --dev --features macros,rt` Add runtime for tests
- `.>` `cargo new avionics-mock` Add the `avionics-mock` application
- `./avionics-mock>` `cargo add --path ../protocols/` Add the protocol library as a dependency
- `./avionics-mock>` `cargo add tokio --features macros,rt-multi-thread` Add the runtim
- `./avionics-mock>` `cargo add tokio --features time` as well as the utility to wait for time
- `.>` `cargo new flight-computer` Now we add the flight computer with its dependencies
- `./flight-computer>` `cargo add --path ../protocols/`
- `./flight-computer>` `cargo add tokio --features macros,rt-multi-thread`
- `./flight-computer>` `cargo add tokio --features time`
- `./flight-computer>` `cargo run --bin flight-computer` Run both components and see it working
- `./flight-computer>` `cargo run --bin avionics-mock`
- `./flight-computer>` `cargo test --package avionics-mock --bin avionics-mock -- sim::tests --nocapture` Test the simulation. Note this can be clicked in VSCode
- `.>` `cargo add --package protocols --dev tokio --features time` For tests. You can also specify --package
