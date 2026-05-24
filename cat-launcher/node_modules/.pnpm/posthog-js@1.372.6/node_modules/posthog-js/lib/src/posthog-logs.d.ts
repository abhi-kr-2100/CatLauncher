import { PostHog } from './posthog-core';
import type { CaptureLogOptions, RemoteConfig, Logger } from './types';
import { Extension } from './extensions/types';
export declare class PostHogLogs implements Extension {
    private readonly _instance;
    private _isLogsEnabled;
    private _isLoaded;
    private readonly _logger;
    private _logBuffer;
    private _flushTimeout;
    private _logger_instance;
    private _intervalLogCount;
    private _intervalWindowStart;
    private _droppedWarned;
    constructor(_instance: PostHog);
    initialize(): void;
    onRemoteConfig(response: RemoteConfig): void;
    reset(): void;
    loadIfEnabled(): void;
    captureLog(options: CaptureLogOptions): void;
    get logger(): Logger;
    flushLogs(transport?: 'XHR' | 'fetch' | 'sendBeacon'): void;
    private _scheduleFlush;
    private _getSdkContext;
}
