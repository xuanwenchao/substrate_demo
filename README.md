# 1. 为 proof of existence (poe) 模块的可调用函数 create_claim, revoke_claim, transfer_claim 添加 benchmark 用例，并且将 benchmark 运行的结果应用在可调用函数上；
代码地址：https://github.com/xuanwenchao/substrate_demo/tree/benchmarking/pallets/poe

运行效果图：
<img width="390" alt="截屏2023-06-23 12 58 55" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/83d34c11-040d-44b3-b59e-16eb586f663c">


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
生成weights的文件：https://github.com/xuanwenchao/substrate_demo/blob/benchmarking/pallets/poe/src/weights.rs

# 2. 选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）

# 2.1 生成aura-chain-spec文件

```j

./target/release/node-template build-spec --chain staging > aura.json
```
生成aura.json文件位置：https://github.com/xuanwenchao/substrate_demo/blob/benchmarking/aura.json

<img width="986" alt="截屏2023-06-25 14 28 35" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/f94808d6-ec67-40e4-b164-2d794c6534e0">

```j

./target/release/node-template build-spec --chain=aura.json --raw > aura-raw.json
```
<img width="991" alt="截屏2023-06-25 15 16 40" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/a701420e-ab06-4d77-bf74-1c5c94ca5b0a">
生成aura-raw.json文件位置：https://github.com/xuanwenchao/substrate_demo/blob/benchmarking/aura-raw.json

consensus_aura和consensus_babe转换代码修改参考：https://github.com/kaichaosun/substrate-stencil/commit/e0a7aaf17e2e003ce80cf8062005be202c6cb017

