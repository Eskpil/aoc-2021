#!/bin/sh

cargo build --release
target/release/day-twentyfour $1 puzzle.input
