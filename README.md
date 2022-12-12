# Rust Graphics Experiment

Essentially following [rustwasm](https://git2hub.com/rustwasm/wasm-bindgen/tree/main/examples/webgl)'s example guide, as well as [tjcdeveloper/evolution-sim](https://github.com/tjcdeveloper/evolution-sim/blob/main/src/manager.rs) to learn the complicated syntax. Essentially handling logic in `Rust` and exporting the code into `wasm` for the browser and render using `canvas` (2d context).

# Development

To start, run

```bash
npm run
```

To install node packages required. Next, run

```bash
cargo build
```

To install the rust crates required, as well as to build the source. Finally, to start the local server, run

```bash
npm run serve
```

Then visit http://localhost:8080 in the browser.
