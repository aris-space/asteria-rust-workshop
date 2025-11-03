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

