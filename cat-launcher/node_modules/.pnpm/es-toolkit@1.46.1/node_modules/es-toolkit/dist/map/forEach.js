'use strict';

Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });

function forEach(map, callback) {
    for (const [key, value] of map) {
        callback(value, key, map);
    }
}

exports.forEach = forEach;
