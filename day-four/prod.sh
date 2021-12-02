#!/bin/sh

cargo build --release
target/release/day-four $1 puzzle.input
