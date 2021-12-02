#!/bin/sh

cargo build --release
target/release/day-two $1 puzzle.input
