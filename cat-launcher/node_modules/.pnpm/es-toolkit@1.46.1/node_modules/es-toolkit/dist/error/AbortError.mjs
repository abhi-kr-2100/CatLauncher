import { DOMException } from '../_internal/DOMException.mjs';

class AbortError extends DOMException {
    constructor(message = 'The operation was aborted') {
        super(message);
    }
}

export { AbortError };
