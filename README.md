## 1. 链上的随机数与链下的随机数的区别：

1.1  链上的随机数：
生成方式：链上的随机数是通过区块链网络的共识算法生成的，如基于区块哈希或区块头的哈希计算。
公开和验证性：链上的随机数是公开的，每个参与者都可以验证其有效性，因为随机数是由全网节点共同生成的。
无法预测性：链上的随机数是不可预测的，因为它们取决于区块链网络的共识过程，每个区块的随机数都是动态变化的。

链上的随机数适用于需要公平性和去中心化的应用场景，例如随机选举、竞拍、游戏等。它们的生成是由区块链网络的共识算法保证的，每个参与者都能够验证其有效性和公正性。

1.2  链下的随机数：
生成方式：链下的随机数是在链外计算环境中生成的，例如使用本地计算机或外部服务提供的随机数生成算法。
控制和可预测性：链下的随机数生成过程可以由开发者完全控制，可以使用特定的随机算法或外部随机源。因此，开发者可以预测和控制随机数的生成结果。
可信度：链下的随机数可能缺乏去中心化的特性，开发者需要确保使用可信的随机数生成器，以防止潜在的安全风险。
在使用上的区别：

链下的随机数适用于开发者需要精确控制或预测随机数生成结果的场景。开发者可以选择合适的随机数算法或外部随机源来生成随机数，并在链下环境中使用。



## 2. 在Offchain Worker中，使用Offchain Indexing特性实现从链上向Offchain Storage中写入数据
代码位置：https://github.com/xuanwenchao/substrate_demo/tree/OCW_Demo/pallets/oci
### 2.1 Cargo test 测试用例执行结果:
<img width="818" alt="截屏2023-06-11 11 49 08" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/390b55dd-2bb6-4f39-aa04-e8db9acc9160">

### 2.2 向Storage中读写数据测试结果

首先运行如下命令启动节点：
```js
./target/release/node-template --dev --enable-offchain-indexing true
```

选择offchainIndexingModule模块 先择setOnChainData,输入数据并提交交易

![截屏2023-06-11 11 27 57](https://github.com/xuanwenchao/substrate_demo/assets/1876277/1f8114e2-ae1d-43ef-a707-8213842584ec)


交易成功后，在命令行可以看到输出的LOG信息

![1686455799910](https://github.com/xuanwenchao/substrate_demo/assets/1876277/b13caf55-d132-4fce-bafb-a6c2c6fe2ee8)


