#!/bin/bash

# fail on error
set -e

if [[ $# -eq 0 ]] ; then
    echo "Server missing"
    exit 1
fi

# update deps
cargo update

cargo build --release

rm -rf /tmp/deploy
mkdir /tmp/deploy

cp Rocket.toml /tmp/deploy
cp target/release/sebmalek-com /tmp/deploy

rsync --progress -rvzh -e 'ssh -o StrictHostKeyChecking=no' /tmp/deploy/* $1:/opt/sebmalek-com
