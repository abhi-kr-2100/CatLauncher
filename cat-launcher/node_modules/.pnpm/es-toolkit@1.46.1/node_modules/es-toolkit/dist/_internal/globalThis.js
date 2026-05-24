'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

const globalThis_ = (typeof globalThis === 'object' && globalThis) ||
    (typeof window === 'object' && window) ||
    (typeof self === 'object' && self) ||
    (typeof global === 'object' && global) ||
    (function () {
        return this;
    })() ||
    Function('return this')();

exports.globalThis = globalThis_;
