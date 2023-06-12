"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.zipAll = void 0;
const zip_1 = require("../observable/zip");
const joinAllInternals_1 = require("./joinAllInternals");
function zipAll(project) {
    return (0, joinAllInternals_1.joinAllInternals)(zip_1.zip, project);
}
exports.zipAll = zipAll;
//# sourceMappingURL=zipAll.js.map