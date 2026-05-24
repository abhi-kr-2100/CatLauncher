'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

function forEach(set, callback) {
    for (const value of set) {
        callback(value, value, set);
    }
}

exports.forEach = forEach;
