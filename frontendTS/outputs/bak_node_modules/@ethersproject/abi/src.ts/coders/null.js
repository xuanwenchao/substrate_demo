"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.NullCoder = void 0;
const abstract_coder_1 = require("./abstract-coder");
class NullCoder extends abstract_coder_1.Coder {
    constructor(localName) {
        super("null", "", localName, false);
    }
    defaultValue() {
        return null;
    }
    encode(writer, value) {
        if (value != null) {
            this._throwError("not null", value);
        }
        return writer.writeBytes([]);
    }
    decode(reader) {
        reader.readBytes(0);
        return reader.coerce(this.name, null);
    }
}
exports.NullCoder = NullCoder;
//# sourceMappingURL=null.js.map