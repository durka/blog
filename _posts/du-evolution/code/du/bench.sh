#!/bin/bash

NWARM=10
NITER=50
DIR=/Users/alex/.cargo

for n in 1 2 3; do
    echo "Compiling v$n..."
    cargo build --release --bin v$n
    echo -n "Warming up v$n..."
    for i in $(seq $NWARM); do
        target/release/v$n $DIR >/dev/null 2>&1
    done
    echo ""
    echo -n "Timing v$n..."
    for j in $(seq $NITER); do
        /usr/bin/time -p target/release/v$n $DIR 2>&1| rg real | awk '{print $2}'
    done | st 2>/dev/null | rg 'mean|stddev'
    echo ""
done

