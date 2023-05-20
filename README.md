# this is a demo to learn base on substrate-node-template

## Firstly, switch branch to learn_demo

## expanded pallet includes:

### 1. extend-pallet：  
//在pallet中调用pallet_contracts的pallet  

### 2. log-test：  
//关于打印Log信息的多种使用方式  

### 3. ocw-test：  
//offchain worker 中使用 签名交易函数的调用 和 未签名交易函数的调用  

### 4. storage-provider：  
//为pallet实现自定义的trait类型(StorageInterface)   

### 5. use-other-pallet：  
//使用自定义的trait类型(StorageInterface)做为存储约束  
//runtime中将类型的实现指向storage-provider的pallet  
//type MyStorage = StorageProviderModule;  
	

### 6 signed-payload-demo：  
//提交仅内容签名的、交易未签名的函数  

### 7. poe
Write unit tests for the dispatchable functions in PoE pallet include
create_claim
revoke_claim
transfer_claim


