#!/bin/sh

cargo build --release
target/release/day-five $1 puzzle.input
