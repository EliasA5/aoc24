#!/bin/bash

if [[ -z $1 ]]; then
    echo "please select a day number"
    exit 1
fi

i=$1

cargo init --vcs git day${i}
cd day${i}
rm -rf day${i}/.git/
cp ../skeleton/src/main.rs src/main.rs



