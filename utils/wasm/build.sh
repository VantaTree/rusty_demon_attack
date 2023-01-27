#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown --release
cd ../../
rm -rf static
mkdir static
cp target/wasm32-unknown-unknown/release/rusty_demon_attack.wasm static/
cp utils/wasm/index.html static/
cp utils/wasm/mq_js_bundle.js static/
mkdir static/resources
cp -ar resources static/
ls -lh static