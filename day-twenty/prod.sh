#!/bin/sh

cargo build --release
target/release/day-twenty $1 puzzle.input
