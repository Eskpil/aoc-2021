#!/bin/sh

cargo build --release
target/release/day-eightteen $1 puzzle.input
