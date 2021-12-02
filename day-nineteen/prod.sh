#!/bin/sh

cargo build --release
target/release/day-nineteen $1 puzzle.input
