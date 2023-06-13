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


## 3. 使用 js sdk 从浏览器frontend获取到前面写入Offchain Storage的数据

本地没有HttpServer代理服务器，我用TypeScript实现后，在控制台输出的方式来展示

代码位置：https://github.com/xuanwenchao/substrate_demo/blob/OCW_Demo/frontendTS/main.ts

首先启动节点

<img width="1347" alt="截屏2023-06-13 09 09 58" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/bfa03663-5a3f-447e-9ecf-b27008e4ea65">

然后执行 ts-node main.ts 执行TypeScript代码，反复检查LocalStorage是否为空，如果不为空则读取内容后终止
<img width="1123" alt="截屏2023-06-13 09 11 57" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/e99ad240-6d0d-4bf6-985f-78c9bb252229">

打开https://polkadot.js.org/apps/#/extrinsics 使用Offchain Indexing特性实现从链上向Offchain Storage中写入数据

![截屏2023-06-13 09 08 48](https://github.com/xuanwenchao/substrate_demo/assets/1876277/5705e96c-f706-401d-83e5-1898beafef88)

提交成功后，查看TypeScript执行的输出

<img width="1132" alt="截屏2023-06-13 09 08 59" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/4669db78-b08b-427c-b6f5-e13d72c58afb">

## 4 设计一个场景实例 (比如获取一个外部的价格信息)，实现从OCW中向链上发起带签名负载的不签名交易，并在Runtime中正确处理
代码位置：https://github.com/xuanwenchao/substrate_demo/tree/OCW_Demo/pallets/ocw-test

首先在offchain_worker方法中请求接口获取价格
接口地址：http://mock.apistub.cn/user/gh_1876277/SampleProject/getprice?who=xuan
得到价格之后，调用send_unsigned_transaction并对内容进行签名，然后调用on-chain的pallet::call方法unsigned_extrinsic_with_signed_payload
unsigned_extrinsic_with_signed_payload方法会将得到的价格写入pallet::storage变量Something
<img width="1131" alt="截屏2023-06-13 21 02 01" src="https://github.com/xuanwenchao/substrate_demo/assets/1876277/13853350-c3bf-4573-9cf5-29781b50beb1">

返回成功信息后，在https://polkadot.js.org/apps/#/chainstate可以验证结果
![截屏2023-06-13 21 02 22](https://github.com/xuanwenchao/substrate_demo/assets/1876277/73acc68d-45fd-4181-b93a-a614d727448e)



