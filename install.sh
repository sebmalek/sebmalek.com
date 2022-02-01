#!/bin/bash

# fail on error
set -e

rm -rf /opt/website
mkdir /opt/website

cp -R static /opt/website/static
cp -R templates /opt/website/templates

cargo build --release
cp target/release/website /opt/website/website
