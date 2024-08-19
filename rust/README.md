Rust code for clia.cc

## Build

```bash
$ cargo build --profile wasm-release --target wasm32-unknown-unknown

$ wasm-bindgen --out-name playground \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/rust.wasm

$ wasm-opt -Oz --output optimized.wasm wasm/target/playground_bg.wasm

$ mv optimized.wasm wasm/target/playground_bg.wasm
```

See: [https://github.com/bevyengine/bevy/tree/main/examples](https://github.com/bevyengine/bevy/tree/main/examples)

See: [https://github.com/bevyengine/bevy/blob/bd8faa7ae17dcd8b4df2beba28876759fb4fdef5/tools/example-showcase/src/main.rs#L698](https://github.com/bevyengine/bevy/blob/bd8faa7ae17dcd8b4df2beba28876759fb4fdef5/tools/example-showcase/src/main.rs#L698)
