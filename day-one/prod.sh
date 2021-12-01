#!/bin/sh

cargo build --release
target/release/day-one $1 puzzle.input
