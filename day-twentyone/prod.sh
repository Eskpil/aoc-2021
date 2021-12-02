#!/bin/sh

cargo build --release
target/release/day-twentyone $1 puzzle.input
