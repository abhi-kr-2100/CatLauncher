import { globalThis as globalThis_ } from './globalThis.mjs';

const DOMException = typeof globalThis_.DOMException !== 'undefined'
    ? globalThis_.DOMException
    : Error;

export { DOMException };
