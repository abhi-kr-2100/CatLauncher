'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

const DOMException = require('../_internal/DOMException.js');

class TimeoutError extends DOMException.DOMException {
    constructor(message = 'The operation was timed out') {
        super(message);
    }
}

exports.TimeoutError = TimeoutError;
