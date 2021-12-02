#!/bin/sh

cargo build --release
target/release/day-twentythree $1 puzzle.input
