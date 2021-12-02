#!/bin/sh

cargo build --release
target/release/day-fourteen $1 puzzle.input
