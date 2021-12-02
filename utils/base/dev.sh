#!/bin/sh

cargo build 
target/debug/@@ $1 puzzle.input
