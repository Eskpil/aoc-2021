#!/bin/sh

cargo build --release
target/release/@@ $1 puzzle.input
