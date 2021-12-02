#!/bin/sh

filename=~/.credentials/aoc-2021
wget --header "Cookie: $(cat $filename)" -O "puzzle.input" "https://adventofcode.com/2021/day/$1/input"
