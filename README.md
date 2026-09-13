# Snow UI Web Example

This page provides browser versions of the five Rust examples: click, clock,
login, lovely girl, and timer. It exercises the `web-sys` renderer on
`wasm32-unknown-unknown`.

Build the release WASM module and generate versioned browser bindings from the
workspace root:

```bash
./build-web.sh
```

The script uses the current Git commit as the asset version and writes the JS
and WASM files under `pkg/<commit>/`. Deploy the generated `index.html` and
the corresponding `pkg/<commit>/` directory together. Old versioned asset
directories can remain cacheable indefinitely.

Serve the example over HTTP:

```bash
python3 -m http.server 8080 --directory .
```

Open <http://127.0.0.1:8080> and verify the five example sections in
`#snow-root`. The click and login buttons update their sections, while the
clock and timer update once per second.