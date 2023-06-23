```j
cargo build --release --features runtime-benchmarks

./ target/release/node-template benchmark pallet \
--chain dev \
--execution wasm \
-wasm-execution compiled \
--pallet pallet_poe --extrinsic "*" I
--steps 20  --repeat 10 \
--output ./pallets/poe/src/weights.rs\
--template .maintain/frame-weight-template.hbs
```