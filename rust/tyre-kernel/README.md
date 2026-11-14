# LibreCorsa Rust tyre kernel

This crate is the first isolated Rust/WASM compute kernel for the VF26 tyre model. It deliberately does not replace the stateful TypeScript tyre simulation: thermal nodes, carcass relaxation, wear, wetness, pressure and contact-patch state remain owned by the existing model.

## Build and test

```sh
cargo test --manifest-path rust/tyre-kernel/Cargo.toml
cargo build --manifest-path rust/tyre-kernel/Cargo.toml --release --target wasm32-unknown-unknown
```

The WASM ABI exports linear memory, `magic_formula`, and `magic_formula_batch`. The batch ABI accepts packed `[x, B, C, E]` f64 tuples and overwrites each tuple's first slot with its result. Batch calls are intentional: crossing the JS/WASM boundary for each individual wheel calculation can cost more than the arithmetic being accelerated.

**Integration status:** kernel source and native parity tests are committed first. It is not enabled in the game until the WASM build and an in-browser worker benchmark demonstrate parity and a measurable win on target hardware. The TypeScript implementation remains authoritative and unchanged.
