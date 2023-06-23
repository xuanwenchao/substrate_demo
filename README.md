# 1. 为 proof of existence (poe) 模块的可调用函数 create_claim, revoke_claim, transfer_claim 添加 benchmark 用例，并且将 benchmark 运行的结果应用在可调用函数上；
代码地址：

运行效果图：


命令行：
```j
cargo build --release --features runtime-benchmarks

./target/release/node-template benchmark pallet \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_poe \
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--json-file=raw.json \
--output ./pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs
```