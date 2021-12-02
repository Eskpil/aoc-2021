#!/bin/sh

cargo build --release
target/release/day-ten $1 puzzle.input
