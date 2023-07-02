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

# 2.2 生成babe-chain-spec文件
代码地址：https://github.com/xuanwenchao/substrate_demo/tree/consensus_babe

```j

./target/release/node-template build-spec --chain substrate_babe > babe.json
./target/release/node-template build-spec --chain=babe.json --raw > babe-raw.json
```
<img width="1002" alt="截屏2023-07-02 13 57 31" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/4c925253-78fa-4e2d-9e20-b69dca4315c8">


consensus_aura和consensus_babe转换代码修改参考：https://github.com/kaichaosun/substrate-stencil/commit/e0a7aaf17e2e003ce80cf8062005be202c6cb017

babe修改后源码参考：https://github.com/kaichaosun/substrate-stencil/blob/840a9fcf7b2e23b417205b4c957297217c1dc43e/node/src/command.rs

# 3. 本地多节点部署
## 3.1 Generate Sr25519 for Baba/Aura
```j

./target/release/node-template key generate --scheme Sr25519 --password-interactive

Key password:
Secret phrase:       produce pioneer review wine two era regret spread old note menu custom
  Network ID:        substrate
  Secret seed:       0x22330b6bdcd03d8606a3467b7f9359a639dcf3961e9ace9219370a821ad1200e
  Public key (hex):  0x08fec9550319844ecd0a980ebd902f666085e2a8a1ad27d96e7c177eb1cd0d2f
  Account ID:        0x08fec9550319844ecd0a980ebd902f666085e2a8a1ad27d96e7c177eb1cd0d2f
  Public key (SS58): 5CGVvknFSSxMgwtnY6h6pYHSvnEukYn7nnJXxQwB3cEmmZKR
  SS58 Address:      5CGVvknFSSxMgwtnY6h6pYHSvnEukYn7nnJXxQwB3cEmmZKR
```

## 3.2 Generate Ed25519 for grandpa
```j
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "pig giraffe ceiling enter weird liar orange decline behind total despair fly"

Key password:
Secret phrase:       pig giraffe ceiling enter weird liar orange decline behind total despair fly
  Network ID:        substrate
  Secret seed:       0xf6be36f6b85a8d1aa0059fcd43032e25e2fca7812bb362b56f13623e95ce4d51
  Public key (hex):  0xbf0dfade67ffc211b47579ac247602e1fdc465f4a8e6317f446660e1ac2463cc
  Account ID:        0xbf0dfade67ffc211b47579ac247602e1fdc465f4a8e6317f446660e1ac2463cc
  Public key (SS58): 5GPDA1DYczVGp1YxDbKkPBPZk5uQzR2TxgJzF9FNXUS5rvgC
  SS58 Address:      5GPDA1DYczVGp1YxDbKkPBPZk5uQzR2TxgJzF9FNXUS5rvgC
```

## 3.3 Generate Babe chain spec file
```j

./target/release/node-template build-spec --disable-default-bootnode --chain substrate_babe > babe.json
```

## 3.4 modify session-keys/babe1
```
{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
        "babe",
        "produce pioneer review wine two era regret spread old note menu custom//1//babe",
        "0x08fec9550319844ecd0a980ebd902f666085e2a8a1ad27d96e7c177eb1cd0d2f"
    ]
}
```

## 3.5 启动主节点
```j

./target/release/node-template --chain=./BabeRaw.json  --base-path /tmp/bootnode --name bootnode
```

## 3.6 Insert the babe secret key
```j

./target/release/node-template key insert --base-path /tmp/bootnode \
  --chain BabeRaw.json \
  --scheme Sr25519 \
  --suri "produce pioneer review wine two era regret spread old note menu custom" \
  --password-interactive \
  --key-type babe

ls /tmp/bootnode/chains/babe_network/keystore

6261626508fec9550319844ecd0a980ebd902f666085e2a8a1ad27d96e7c177eb1cd0d2f
```

## 3.7 启动Validator节点

```j
 ./target/release/node-template --chain=./BabeRaw.json --validator  --base-path /tmp/validator2 --name validator2
```


