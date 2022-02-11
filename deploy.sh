#!/bin/bash

# fail on error
set -e

if [[ $# -eq 0 ]] ; then
    echo "Server missing"
    exit 1
fi

# update deps
cargo update

cargo build --release -j6

rm -rf /tmp/deploy
mkdir /tmp/deploy

cp Rocket.toml /tmp/deploy
cp target/release/sebmalek-com /tmp/deploy
cp -R static/ /tmp/deploy
cp -R templates/ /tmp/deploy

rsync --progress -rvzh --exclude={'.gitkeep'} -e 'ssh -o StrictHostKeyChecking=no' /tmp/deploy/* $1:/opt/sebmalek-com

ssh -t $1 "sudo systemctl restart sebmalek-com.service"
