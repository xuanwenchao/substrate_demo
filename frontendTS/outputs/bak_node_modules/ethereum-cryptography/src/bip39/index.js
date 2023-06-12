"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.mnemonicToSeedSync = exports.mnemonicToSeed = exports.validateMnemonic = exports.entropyToMnemonic = exports.mnemonicToEntropy = exports.generateMnemonic = void 0;
const bip39 = require("../vendor/bip39-without-wordlists");
function generateMnemonic(wordlist, strength = 128) {
    return bip39.generateMnemonic(strength, undefined, wordlist);
}
exports.generateMnemonic = generateMnemonic;
function mnemonicToEntropy(mnemonic, wordlist) {
    return bip39.mnemonicToEntropy(mnemonic, wordlist);
}
exports.mnemonicToEntropy = mnemonicToEntropy;
function entropyToMnemonic(entropy, wordlist) {
    return bip39.entropyToMnemonic(entropy, wordlist);
}
exports.entropyToMnemonic = entropyToMnemonic;
function validateMnemonic(mnemonic, wordlist) {
    return bip39.validateMnemonic(mnemonic, wordlist);
}
exports.validateMnemonic = validateMnemonic;
function mnemonicToSeed(mnemonic, passphrase = "") {
    return __awaiter(this, void 0, void 0, function* () {
        return bip39.mnemonicToSeed(mnemonic, passphrase);
    });
}
exports.mnemonicToSeed = mnemonicToSeed;
function mnemonicToSeedSync(mnemonic, passphrase = "") {
    return bip39.mnemonicToSeedSync(mnemonic, passphrase);
}
exports.mnemonicToSeedSync = mnemonicToSeedSync;
//# sourceMappingURL=index.js.map