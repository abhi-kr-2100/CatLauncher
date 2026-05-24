import { DOMException } from '../_internal/DOMException.mjs';

/**
 * An error class representing a timeout operation.
 * @augments DOMException
 */
declare class TimeoutError extends DOMException {
    constructor(message?: string);
}

export { TimeoutError };
