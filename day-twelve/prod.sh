#!/bin/sh

cargo build --release
target/release/day-twelve $1 puzzle.input
