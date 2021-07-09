This package provides web assembly bindings for laz-rs (currently only laz decompression).

To get started:

```
cargo build
wasm-pack build --target web
python3 -m http.server --bind 127.0.0.1
```

Then, you can navigate to `examples/wasm.html` and can view the output in the console.
