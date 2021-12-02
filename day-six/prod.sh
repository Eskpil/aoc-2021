#!/bin/sh

cargo build --release
target/release/day-six $1 puzzle.input
