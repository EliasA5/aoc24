#!/bin/bash

if [[ -z $1 || -z $2 ]]; then
    echo "please send me your seed"
    exit 1
fi

mkdir -p day$i/input/
curl --cookie $2 https://adventofcode.com/2024/day/$1/input > day$i/input/full_input
