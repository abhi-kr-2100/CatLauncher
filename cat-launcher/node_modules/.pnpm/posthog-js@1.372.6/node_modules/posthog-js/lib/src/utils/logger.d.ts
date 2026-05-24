import type { Logger } from '@posthog/core';
type CreateLoggerOptions = {
    debugEnabled?: boolean;
};
type PosthogJsLogger = Omit<Logger, 'createLogger'> & {
    _log: (level: 'debug' | 'log' | 'warn' | 'error', ...args: any[]) => void;
    uninitializedWarning: (methodName: string) => void;
    createLogger: (prefix: string, options?: CreateLoggerOptions) => PosthogJsLogger;
};
export declare const logger: PosthogJsLogger;
export declare const createLogger: (prefix: string, options?: CreateLoggerOptions) => PosthogJsLogger;
export {};
