#!/bin/sh

cargo build 
target/debug/utils $1 puzzle.input
