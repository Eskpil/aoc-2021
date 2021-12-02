#!/bin/sh

cargo build --release
target/release/day-three $1 puzzle.input
