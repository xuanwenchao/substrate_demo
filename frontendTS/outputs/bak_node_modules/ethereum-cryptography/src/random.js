"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getRandomBytesSync = exports.getRandomBytes = void 0;
const randombytes = require("randombytes");
function getRandomBytes(bytes) {
    return new Promise((resolve, reject) => {
        randombytes(bytes, function (err, resp) {
            if (err) {
                reject(err);
                return;
            }
            resolve(resp);
        });
    });
}
exports.getRandomBytes = getRandomBytes;
function getRandomBytesSync(bytes) {
    return randombytes(bytes);
}
exports.getRandomBytesSync = getRandomBytesSync;
//# sourceMappingURL=random.js.map