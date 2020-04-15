#!/bin/sh

which cargo || echo 'You need to install rust + cargo: https://www.rust-lang.org/tools/install'

cargo run --release
