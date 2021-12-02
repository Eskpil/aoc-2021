#!/bin/sh

cargo build --release
target/release/day-seven $1 puzzle.input
