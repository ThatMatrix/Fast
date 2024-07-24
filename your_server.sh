#!/bin/sh
exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/fast\
    --manifest-path $(dirname $0)/Cargo.toml -- "$@"
