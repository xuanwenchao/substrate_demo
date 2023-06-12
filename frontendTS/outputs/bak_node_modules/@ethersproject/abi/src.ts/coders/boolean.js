"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BooleanCoder = void 0;
const abstract_coder_1 = require("./abstract-coder");
class BooleanCoder extends abstract_coder_1.Coder {
    constructor(localName) {
        super("bool", "bool", localName, false);
    }
    defaultValue() {
        return false;
    }
    encode(writer, value) {
        return writer.writeValue(value ? 1 : 0);
    }
    decode(reader) {
        return reader.coerce(this.type, !reader.readValue().isZero());
    }
}
exports.BooleanCoder = BooleanCoder;
//# sourceMappingURL=boolean.js.map