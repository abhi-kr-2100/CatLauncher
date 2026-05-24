import { DOMException } from '../_internal/DOMException.js';

/**
 * An error class representing an aborted operation.
 * @augments DOMException
 */
declare class AbortError extends DOMException {
    constructor(message?: string);
}

export { AbortError };
