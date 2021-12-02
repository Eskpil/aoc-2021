#!/bin/sh

cargo build --release
target/release/day-thirteen $1 puzzle.input
