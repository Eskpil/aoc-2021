#!/bin/sh

cargo build --release
target/release/day-eight $1 puzzle.input
