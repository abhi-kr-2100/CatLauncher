"use strict";
var __webpack_require__ = {};
(()=>{
    __webpack_require__.d = (exports1, definition)=>{
        for(var key in definition)if (__webpack_require__.o(definition, key) && !__webpack_require__.o(exports1, key)) Object.defineProperty(exports1, key, {
            enumerable: true,
            get: definition[key]
        });
    };
})();
(()=>{
    __webpack_require__.o = (obj, prop)=>Object.prototype.hasOwnProperty.call(obj, prop);
})();
(()=>{
    __webpack_require__.r = (exports1)=>{
        if ('undefined' != typeof Symbol && Symbol.toStringTag) Object.defineProperty(exports1, Symbol.toStringTag, {
            value: 'Module'
        });
        Object.defineProperty(exports1, '__esModule', {
            value: true
        });
    };
})();
var __webpack_exports__ = {};
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
    gzipCompress: ()=>gzipCompress,
    isGzipSupported: ()=>isGzipSupported,
    isNativeAsyncGzipReadError: ()=>isNativeAsyncGzipReadError
});
function isGzipSupported() {
    return 'CompressionStream' in globalThis && 'TextEncoder' in globalThis && 'Response' in globalThis && 'function' == typeof Response.prototype.blob;
}
const isNativeAsyncGzipReadError = (error)=>{
    if (!error || 'object' != typeof error) return false;
    const name = 'name' in error ? String(error.name) : '';
    return 'NotReadableError' === name;
};
async function gzipCompress(input, isDebug = true, options) {
    try {
        const compressedStream = new CompressionStream('gzip');
        const writer = compressedStream.writable.getWriter();
        const writePromise = writer.write(new TextEncoder().encode(input)).then(()=>writer.close()).catch(async (err)=>{
            try {
                await writer.abort(err);
            } catch  {}
            throw err;
        });
        const responsePromise = new Response(compressedStream.readable).blob();
        const [compressed] = await Promise.all([
            responsePromise,
            writePromise
        ]);
        return compressed;
    } catch (error) {
        if (options?.rethrow) throw error;
        if (isDebug) console.error('Failed to gzip compress data', error);
        return null;
    }
}
exports.gzipCompress = __webpack_exports__.gzipCompress;
exports.isGzipSupported = __webpack_exports__.isGzipSupported;
exports.isNativeAsyncGzipReadError = __webpack_exports__.isNativeAsyncGzipReadError;
for(var __webpack_i__ in __webpack_exports__)if (-1 === [
    "gzipCompress",
    "isGzipSupported",
    "isNativeAsyncGzipReadError"
].indexOf(__webpack_i__)) exports[__webpack_i__] = __webpack_exports__[__webpack_i__];
Object.defineProperty(exports, '__esModule', {
    value: true
});
