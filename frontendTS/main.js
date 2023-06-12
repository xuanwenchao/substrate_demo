var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var _this = this;
var text1 = document.getElementById('text1');
var text2 = document.getElementById('text2');
var _a = require('@polkadot/api'), ApiPromise = _a.ApiPromise, WsProvider = _a.WsProvider;
var xxhashAsHex = require('@polkadot/util-crypto').xxhashAsHex;
// const WEB_SOCKET = 'ws://localhost:9944';
var WEB_SOCKET = 'ws://127.0.0.1:9944';
var sleep = function (ms) { return new Promise(function (resolve) { return setTimeout(resolve, ms); }); };
var connectSubstrate = function () { return __awaiter(_this, void 0, void 0, function () {
    var wsProvider, api;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                text1.textContent = "### connecting to substrate.......";
                wsProvider = new WsProvider(WEB_SOCKET);
                return [4 /*yield*/, ApiPromise.create({ provider: wsProvider, types: {} })];
            case 1:
                api = _a.sent();
                return [4 /*yield*/, api.isReady];
            case 2:
                _a.sent();
                text1.textContent = "### connection to substrate successful!!!";
                console.log("### connection to substrate successful!!!");
                return [2 /*return*/, api];
        }
    });
}); };
//读取offchain storage的内容
var readOffchainStorage = function (api) { return __awaiter(_this, void 0, void 0, function () {
    var count, offchainData, decodedData, struct;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                count = 100;
                _a.label = 1;
            case 1:
                if (!(count > 0)) return [3 /*break*/, 4];
                return [4 /*yield*/, api.rpc.offchain.localStorageGet('PERSISTENT', 'offchain_tx_key_for_test_1234567890')];
            case 2:
                offchainData = _a.sent();
                // console.log('offchainStorage:', offchainData.toHuman());
                if (offchainData.isSome) {
                    decodedData = offchainData.unwrap();
                    console.log('### decodedData:', decodedData);
                    struct = {
                        indexing_data: decodedData.get('indexing_data').toU8a(),
                        block_number: decodedData.get('block_number'),
                    };
                    text2.textContent = '### offchainData: ' + JSON.stringify(struct);
                    console.log('### offchainData:', struct);
                    return [3 /*break*/, 4];
                }
                else {
                    text2.textContent = '### offchainStorage is  empty!!! count=' + count;
                    console.log('### offchainData is empty!!! count=', count);
                }
                count--;
                return [4 /*yield*/, sleep(2000)];
            case 3:
                _a.sent();
                return [3 /*break*/, 1];
            case 4: return [2 /*return*/];
        }
    });
}); };
//subscribe event on node templete
var subscribeEvent = function (api) { return __awaiter(_this, void 0, void 0, function () {
    var eventName;
    return __generator(this, function (_a) {
        eventName = 'templateModule.SomethingStored';
        api.query.system.events(function (events) {
            events.forEach(function (record) {
                var event = record.event;
                if (event.section === 'templateModule') { //订阅指定Module的Event(以templateModule为例)
                    console.log('Received custom event:', event.method, event.toHuman());
                }
            });
        });
        return [2 /*return*/];
    });
}); };
var main = function () { return __awaiter(_this, void 0, void 0, function () {
    var api;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0: return [4 /*yield*/, connectSubstrate()];
            case 1:
                api = _a.sent();
                return [4 /*yield*/, readOffchainStorage(api)];
            case 2:
                _a.sent();
                return [4 /*yield*/, subscribeEvent(api)];
            case 3:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
main()
    .then(function () {
    console.log("successfully exited");
    process.exit(0);
})
    .catch(function (err) {
    console.log('error occur:', err);
    process.exit(1);
});
