import { globalThis as globalThis_ } from '../_internal/globalThis.mjs';

function isBuffer(x) {
    return typeof globalThis_.Buffer !== 'undefined' && globalThis_.Buffer.isBuffer(x);
}

export { isBuffer };
