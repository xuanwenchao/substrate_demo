"use strict";
/*
    This file is part of web3.js.
    web3.js is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    web3.js is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Lesser General Public License for more details.
    You should have received a copy of the GNU Lesser General Public License
    along with web3.js.  If not, see <http://www.gnu.org/licenses/>.
*/
/**
 * @file formatters-test.ts
 * @author Samuel Furter <samuel@ethereum.org>
 * @date 2018
 */
Object.defineProperty(exports, "__esModule", { value: true });
const web3_core_helpers_1 = require("web3-core-helpers");
// $ExpectType number
web3_core_helpers_1.formatters.outputBigNumberFormatter(100);
// $ExpectType string
web3_core_helpers_1.formatters.inputSignFormatter('0x0');
// $ExpectType string
web3_core_helpers_1.formatters.inputAddressFormatter('0x0');
// $ExpectType boolean
web3_core_helpers_1.formatters.isPredefinedBlockNumber('latest');
// $ExpectType string | number
web3_core_helpers_1.formatters.inputBlockNumberFormatter('0x0');
// $ExpectType any
web3_core_helpers_1.formatters.outputBlockFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.txInputFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.inputCallFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.inputTransactionFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.outputTransactionFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.outputTransactionReceiptFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.inputLogFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.outputLogFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.inputPostFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.outputPostFormatter({});
// $ExpectType any
web3_core_helpers_1.formatters.outputSyncingFormatter({});
//# sourceMappingURL=formatters-test.js.map