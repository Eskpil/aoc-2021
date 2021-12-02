#!/bin/sh

cargo build --release
target/release/day-elleven $1 puzzle.input
