
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { xxhashAsHex } = require('@polkadot/util-crypto');


// const WEB_SOCKET = 'ws://localhost:9944';
const WEB_SOCKET = 'ws://127.0.0.1:9944';

const sleep = (ms: number | undefined) => new Promise(resolve => setTimeout(resolve, ms));


const connectSubstrate = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    console.log("### connection to substrate successful!!!")
    return api;
};

// import { Bytes } from '@polkadot/types';

// interface OffchainIndexingData {
//   indexing_data: String;
//   block_number: any;
// }

//读取offchain storage的内容
const readOffchainStorage = async (api) => {
    var count = 100;
    while (count > 0) {


        const offchainData = await api.rpc.offchain.localStorageGet('PERSISTENT', 'offchain_tx_key_for_test_1234567890');
        // console.log('offchainStorage:', offchainData.toHuman());

        if (offchainData.isSome) {
            const decodedData = offchainData.unwrap();
            console.log('### decodedData:', decodedData);
            const indexingDataString = Buffer.from(decodedData).toString('utf8').trim();
            const blockNumber = 1;//decodedData.toJSON().block_number.toNumber();
            console.log('### indexing_data='+ indexingDataString +' blockNumber=',blockNumber);
            break;
        } else {
            console.log('### offchainData is empty!!! count=',count);
        }
        count--;
        await sleep(2000);
    }
}


//subscribe event on node templete
const subscribeEvent = async (api: typeof ApiPromise) => {
    const eventName = 'templateModule.SomethingStored';
    api.query.system.events((events) => {
        events.forEach((record) => {
            const { event } = record;
            if (event.section === 'templateModule') { //订阅指定Module的Event(以templateModule为例)
                console.log('Received custom event:', event.method, event.toHuman());
            }
        });
    });
}




const main = async () => {
    const api = await connectSubstrate();
    await readOffchainStorage(api);
    await subscribeEvent(api);
    // await sleep(6000000);
}



main()
    .then(() => {
        console.log("successfully exited");
        process.exit(0);
    })
    .catch(err => {
        console.log('error occur:', err);
        process.exit(1);
    });


