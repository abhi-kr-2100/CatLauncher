'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

const globalThis = require('./globalThis.js');

const DOMException = typeof globalThis.globalThis.DOMException !== 'undefined'
    ? globalThis.globalThis.DOMException
    : Error;

exports.DOMException = DOMException;
