#!/bin/bash

NWARM=20
NITER=100
DIR=$HOME/.cargo

function go() {
    echo -n "Warming up $2..."
    for i in $(seq $NWARM); do
        echo -n " $i"
        $1 $DIR >/dev/null 2>&1
    done
    echo ""
    echo -n "Timing $2..."
    echo >$2.time
    for j in $(seq $NITER); do
        echo -n " $j"
        /usr/bin/time -p $1 $DIR 2>&1| rg real | awk '{print $2}' >>$2.time
    done
    echo ""
}

go "/usr/bin/du -s" du
for n in 1 2 3; do
    echo "Compiling v$n..."
    cargo build --release --bin v$n
    go target/release/v$n v$n
done

