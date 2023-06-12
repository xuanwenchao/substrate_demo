"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.decrypt = exports.encrypt = void 0;
const browserifyAes = require("browserify-aes");
const SUPPORTED_MODES = ["aes-128-ctr", "aes-128-cbc", "aes-256-cbc"];
function ensureAesMode(mode) {
    if (!mode.startsWith("aes-")) {
        throw new Error(`AES submodule doesn't support mode ${mode}`);
    }
}
function warnIfUnsuportedMode(mode) {
    if (!SUPPORTED_MODES.includes(mode)) {
        // tslint:disable-next-line no-console
        console.warn("Using an unsupported AES mode. Consider using aes-128-ctr.");
    }
}
function encrypt(msg, key, iv, mode = "aes-128-ctr", pkcs7PaddingEnabled = true) {
    ensureAesMode(mode);
    const cipher = browserifyAes.createCipheriv(mode, key, iv);
    cipher.setAutoPadding(pkcs7PaddingEnabled);
    const encrypted = cipher.update(msg);
    const final = cipher.final();
    return Buffer.concat([encrypted, final]);
}
exports.encrypt = encrypt;
function decrypt(cypherText, key, iv, mode = "aes-128-ctr", pkcs7PaddingEnabled = true) {
    ensureAesMode(mode);
    const decipher = browserifyAes.createDecipheriv(mode, key, iv);
    decipher.setAutoPadding(pkcs7PaddingEnabled);
    const encrypted = decipher.update(cypherText);
    const final = decipher.final();
    return Buffer.concat([encrypted, final]);
}
exports.decrypt = decrypt;
//# sourceMappingURL=aes.js.map