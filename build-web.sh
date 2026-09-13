#!/bin/sh
set -eu

version=${WEB_VERSION:-$(git rev-parse --short HEAD)}
wasm_target="target/wasm32-unknown-unknown/release/snow_ui_web_example.wasm"
output_dir="pkg/$version"

rm -rf pkg/*

cargo build \
  --manifest-path Cargo.toml \
  --package snow-ui-web-example \
  --target wasm32-unknown-unknown \
  --release

rm -rf "$output_dir"
mkdir -p "$output_dir"
wasm-bindgen "$wasm_target" \
  --out-dir "$output_dir" \
  --target web

sed "s/__WEB_VERSION__/$version/g" index.html.in > index.html.tmp
mv index.html.tmp index.html