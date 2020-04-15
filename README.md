# Open Space-ial Distancing

TWAus Shokunin challenge April 2020

## Run it

1. [Install rust + cargo](https://www.rust-lang.org/tools/install)
2. `./go.sh` or just `cargo run --release`

## Run the tests

- `cargo test`

## Notable features

- It's fast
    - Mostly just because I picked an awesome, fast, low-level language.
    - The full program, including 10,000 samples of [0.0, 1.0], takes about 0.75 seconds
- It's well tested
    - Most of it was TDD'd.
    - Lots of functions that are just data-in/data-out, which is easy to test
    - Only trickier part was `stats.rs`, because it calculates a single number that depends on a lot of random input
        - The solution was to make a trait (interface) for the maze-solver, and then manually implement a stub that can be forced to succeed/fail

## TODO:

- Visualise the path finding algorithm
