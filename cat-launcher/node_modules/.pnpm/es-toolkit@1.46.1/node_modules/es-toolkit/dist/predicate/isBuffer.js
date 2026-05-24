'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

const globalThis = require('../_internal/globalThis.js');

function isBuffer(x) {
    return typeof globalThis.globalThis.Buffer !== 'undefined' && globalThis.globalThis.Buffer.isBuffer(x);
}

exports.isBuffer = isBuffer;
