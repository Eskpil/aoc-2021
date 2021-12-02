#!/bin/sh

cargo build --release
target/release/day-twentytwo $1 puzzle.input
