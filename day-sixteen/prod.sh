#!/bin/sh

cargo build --release
target/release/day-sixteen $1 puzzle.input
