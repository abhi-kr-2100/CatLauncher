import { isNumber, isObject, isString } from "../utils/index.mjs";
const EXCEPTION_STEP_INTERNAL_FIELDS = {
    MESSAGE: '$message',
    TIMESTAMP: '$timestamp'
};
const RESERVED_EXCEPTION_STEP_KEYS = new Set([
    EXCEPTION_STEP_INTERNAL_FIELDS.MESSAGE,
    EXCEPTION_STEP_INTERNAL_FIELDS.TIMESTAMP
]);
const DEFAULT_EXCEPTION_STEPS_CONFIG = {
    enabled: true,
    max_bytes: 32768
};
function resolveExceptionStepsConfig(config) {
    if (!config) return {
        ...DEFAULT_EXCEPTION_STEPS_CONFIG
    };
    return {
        enabled: config.enabled ?? DEFAULT_EXCEPTION_STEPS_CONFIG.enabled,
        max_bytes: normalizePositiveInteger(config.max_bytes, DEFAULT_EXCEPTION_STEPS_CONFIG.max_bytes)
    };
}
function stripReservedExceptionStepFields(properties) {
    if (!properties) return {
        sanitizedProperties: {},
        droppedKeys: []
    };
    const droppedKeys = [];
    const sanitizedProperties = Object.keys(properties).reduce((acc, key)=>{
        if (RESERVED_EXCEPTION_STEP_KEYS.has(key)) {
            droppedKeys.push(key);
            return acc;
        }
        acc[key] = properties[key];
        return acc;
    }, {});
    return {
        sanitizedProperties,
        droppedKeys
    };
}
class ExceptionStepsBuffer {
    constructor(config){
        this._entries = [];
        this._totalBytes = 0;
        this._config = resolveExceptionStepsConfig(config);
    }
    setConfig(config) {
        this._config = resolveExceptionStepsConfig(config);
        this._trimToMaxBytes();
    }
    add(step) {
        const serialized = normalizeAndSerializeStep(step);
        if (!serialized) return;
        const bytes = getUtf8ByteLength(serialized.json);
        if (bytes > this._config.max_bytes) return;
        this._entries.push({
            step: serialized.step,
            bytes
        });
        this._totalBytes += bytes;
        this._trimToMaxBytes();
    }
    getAttachable() {
        return this._entries.map((e)=>e.step);
    }
    clear() {
        this._entries = [];
        this._totalBytes = 0;
    }
    size() {
        return this._entries.length;
    }
    _trimToMaxBytes() {
        while(this._totalBytes > this._config.max_bytes && this._entries.length > 0){
            const evicted = this._entries.shift();
            if (evicted) this._totalBytes -= evicted.bytes;
        }
    }
}
function normalizePositiveInteger(input, fallback) {
    if (!isNumber(input) || input === 1 / 0 || input === -1 / 0) return fallback;
    const normalized = Math.floor(input);
    if (normalized < 0) return fallback;
    return normalized;
}
function normalizeAndSerializeStep(step) {
    const json = safeStringify(step);
    if (!json) return;
    try {
        const parsed = JSON.parse(json);
        if (!isObject(parsed)) return;
        const parsedStep = parsed;
        const message = parsedStep[EXCEPTION_STEP_INTERNAL_FIELDS.MESSAGE];
        const timestamp = parsedStep[EXCEPTION_STEP_INTERNAL_FIELDS.TIMESTAMP];
        if (!isString(message) || 0 === message.trim().length) return;
        if (!isString(timestamp) && !isNumber(timestamp)) return;
        return {
            step: parsedStep,
            json
        };
    } catch  {
        return;
    }
}
function safeStringify(value) {
    const seen = new WeakSet();
    try {
        return JSON.stringify(value, (_key, replacementValue)=>{
            if ('bigint' == typeof replacementValue) return replacementValue.toString();
            if ('function' == typeof replacementValue || 'symbol' == typeof replacementValue) return;
            if (replacementValue instanceof Date) return replacementValue.toISOString();
            if (replacementValue instanceof Error) return {
                name: replacementValue.name,
                message: replacementValue.message,
                stack: replacementValue.stack
            };
            if (replacementValue && 'object' == typeof replacementValue) {
                if (seen.has(replacementValue)) return '[Circular]';
                seen.add(replacementValue);
            }
            return replacementValue;
        });
    } catch  {
        return;
    }
}
function getUtf8ByteLength(value) {
    if ('undefined' != typeof TextEncoder) return new TextEncoder().encode(value).length;
    const encoded = encodeURIComponent(value);
    let byteLength = 0;
    for(let i = 0; i < encoded.length; i++)if ('%' === encoded[i]) {
        byteLength += 1;
        i += 2;
    } else byteLength += 1;
    return byteLength;
}
export { DEFAULT_EXCEPTION_STEPS_CONFIG, EXCEPTION_STEP_INTERNAL_FIELDS, ExceptionStepsBuffer, getUtf8ByteLength, resolveExceptionStepsConfig, stripReservedExceptionStepFields };
