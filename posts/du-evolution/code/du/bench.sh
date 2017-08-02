#!/bin/bash

NWARM=25
NITER=1000
DIR="~/Documents/research/proton/code/nri/data/20170716/stickcam/1/"

for n in 1 2 3; do
    CMD="cargo run --release --bin v$n $DIR"
    for i in $(seq 25); do
        $CMD >/dev/null 2>&1
    done
    for i in $(seq 1000); do
        /usr/bin/time -p $CMD 2>&1| rg real | awk '{print $2}'
    done | st 2>/dev/null | rg 'mean|stddev'
done

