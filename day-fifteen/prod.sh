#!/bin/sh

cargo build --release
target/release/day-fifteen $1 puzzle.input
