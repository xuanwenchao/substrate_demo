var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { xxhashAsHex } = require('@polkadot/util-crypto');
// const WEB_SOCKET = 'ws://localhost:9944';
const WEB_SOCKET = 'ws://127.0.0.1:9944';
const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));
const connectSubstrate = () => __awaiter(this, void 0, void 0, function* () {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = yield ApiPromise.create({ provider: wsProvider, types: {} });
    yield api.isReady;
    console.log("### connection to substrate successful!!!");
    return api;
});
//读取offchain storage的内容
const readOffchainStorage = (api) => __awaiter(this, void 0, void 0, function* () {
    var count = 100;
    while (count > 0) {
        const offchainData = yield api.rpc.offchain.localStorageGet('PERSISTENT', 'offchain_tx_key_for_test_1234567890');
        // console.log('offchainStorage:', offchainData.toHuman());
        if (offchainData.isSome) {
            console.log('### offchainData:', offchainData.toHuman());
            break;
        }
        else {
            console.log('### offchainData is empty!!! count=', count);
        }
        count--;
        yield sleep(2000);
    }
});
//subscribe event on node templete
const subscribeEvent = (api) => __awaiter(this, void 0, void 0, function* () {
    const eventName = 'templateModule.SomethingStored';
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (event.section === 'templateModule') { //订阅指定Module的Event(以templateModule为例)
                console.log('Received custom event:', event.method, event.toHuman());
            }
        });
    });
});
const main = () => __awaiter(this, void 0, void 0, function* () {
    const api = yield connectSubstrate();
    yield readOffchainStorage(api);
    yield subscribeEvent(api);
    yield sleep(6000000);
});
main()
    .then(() => {
    console.log("successfully exited");
    process.exit(0);
})
    .catch(err => {
    console.log('error occur:', err);
    process.exit(1);
});
//# sourceMappingURL=main.js.map