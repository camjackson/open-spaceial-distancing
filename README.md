# Open Space-ial Distancing

TWAus Shokunin challenge April 2020

## Run it

1. [Install rust + cargo](https://www.rust-lang.org/tools/install)
2. `./go.sh` or just `cargo run --release`

Example output:

```
$ cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/open-spaceial-distancing`
Number of samples for each p: 10000
  p | safety percentage
0.0 | 100.0%
0.1 | 99.44%
0.2 | 95.54%
0.3 | 77.43%
0.4 | 35.63%
0.5 | 5.9%
0.6 | 0.51%
0.7 | 0.01%
0.8 | 0.0%
0.9 | 0.0%
1.0 | 0.0%
```

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
- No libraries, except for the standard `rand` crate which I don't think counts

## Bonus feature

You can visualise the path-finding algorithm! Open up main.rs, and set `print_path: true`. This will
make it print every office after trying to find a way out, so you probably also want to also set `sample_count` to something low (i.e. less than 10) or your terminal will be super messy. Here's an
example of a visualised office, where:

- `0` = empty desk
- `x` = occupied desk
- `-` = part of the path taken to get out

```
-x00xxx00x
-x---0x00x
-0-x--00x0
----x--00x
-xx--x-000
x-x-x--0x0
---x---0x0
x---xx-0x0
x-------0x
--x---x---
```

It's a little bit hard to read, but the escapee is essentially hugging every left 'wall' through the 'maze', which is why they take such a roundabout route out!
