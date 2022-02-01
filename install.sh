#!/bin/bash

# fail on error
set -e

rm -rf /opt/sebmalek-com
mkdir /opt/sebmalek-com

cp -R static /opt/sebmalek-com/static
cp -R templates /opt/sebmalek-com/templates

cargo build --release
cp target/release/website /opt/sebmalek-com/website
