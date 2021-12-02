#!/bin/sh

cargo build --release
target/release/day-seventeen $1 puzzle.input
