"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createHashFunction = void 0;
function createHashFunction(hashConstructor) {
    return msg => {
        const hash = hashConstructor();
        hash.update(msg);
        return Buffer.from(hash.digest());
    };
}
exports.createHashFunction = createHashFunction;
//# sourceMappingURL=hash-utils.js.map