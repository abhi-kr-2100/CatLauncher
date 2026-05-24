'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

const DOMException = require('../_internal/DOMException.js');

class AbortError extends DOMException.DOMException {
    constructor(message = 'The operation was aborted') {
        super(message);
    }
}

exports.AbortError = AbortError;
