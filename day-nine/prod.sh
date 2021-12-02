#!/bin/sh

cargo build --release
target/release/day-nine $1 puzzle.input
