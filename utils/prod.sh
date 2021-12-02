#!/bin/sh

cargo build --release
target/release/utils $1 puzzle.input
