import { Logger } from '../types';
import type { PostHogCoreStateless } from '../posthog-core-stateless';
import type { CaptureLogOptions, LogSdkContext, ResolvedPostHogLogsConfig } from './types';
export declare class PostHogLogs {
    private readonly _instance;
    private readonly _config;
    private readonly _logger;
    private readonly _getContext;
    private readonly _onReady;
    private readonly _waitForStoragePersist;
    private _maxBufferSize;
    private _flushIntervalMs;
    private _maxBatchRecordsPerPost;
    private _flushTimer?;
    private _flushPromise;
    private _rateCapWindowMs;
    private _maxLogsPerInterval?;
    private _intervalWindowStart;
    private _intervalLogCount;
    private _droppedWarned;
    constructor(_instance: PostHogCoreStateless, _config: ResolvedPostHogLogsConfig, _logger: Logger, _getContext: () => LogSdkContext, _onReady: (fn: () => void) => void, _waitForStoragePersist?: () => Promise<void>);
    captureLog(options: CaptureLogOptions): void;
    /**
     * Runs the configured `beforeSend` hook(s) on a capture record:
     *   - single fn OR array of fns (chain, left-to-right)
     *   - returning `null` drops the record (logged at info)
     *   - a thrown error is logged and the chain *continues* with the previous
     *     result — a buggy user filter must never crash the caller's
     *     `captureLog()` call
     */
    private _runBeforeSend;
    /**
     * Returns `true` if this capture fits within the current rate-cap window,
     * `false` if it should be dropped.
     *
     * Fixed (tumbling) window: the counter resets the first time `captureLog`
     * fires after `rateCapWindowMs` has elapsed — no timer needed.
     * `maxLogsPerInterval === undefined` means unbounded.
     *
     * Wall-clock safety: if `Date.now()` jumps backward (manual device-clock
     * change, big NTP correction), `elapsed` goes negative. We treat that the
     * same as "window expired" and reset — otherwise the rate cap would be
     * stuck until the clock caught up to the old window start, potentially
     * dropping logs for hours.
     *
     * Pre-init note: the counter increments here, before `_onReady` defers
     * `_enqueue` to the init promise. If init resolves slowly and the user is
     * later opted out, the counter has already consumed budget for records
     * that won't enqueue. Cosmetic — no record is "lost" beyond what's
     * already gated, and the window rolls on its own.
     */
    private _checkRateLimit;
    /**
     * Drains `LogsQueue` in `maxBatchRecordsPerPost` slices, POSTing each as an
     * OTLP payload.
     *   - Network error   → keep items in queue, re-throw (caller retries later)
     *   - 413             → halve batch size, retry same records (do not advance)
     *   - Any other error → drop the batch (avoid infinite loop on malformed data),
     *                       re-throw so callers can log/report
     * Concurrent calls are serialized through `_flushPromise` so records at the
     * head of the queue can't be sent twice.
     */
    flush(): Promise<void>;
    private _flushInner;
    private _persistQueueAdvance;
    /**
     * OTLP resource attributes for every batch.
     *
     * Layout: user `resourceAttributes` spread first, then SDK-controlled
     * keys layered on top so users cannot accidentally clobber them. Most logs
     * backends index on `service.name` and `telemetry.sdk.*` for routing,
     * SDK-version dashboards, and bug-correlation; letting a stray user key
     * overwrite them silently breaks ingestion attribution. The dedicated
     * `serviceName` / `environment` / `serviceVersion` config fields are the
     * supported way to override `service.name` / `deployment.environment` /
     * `service.version`.
     */
    private _buildResourceAttributes;
    private _enqueue;
    /**
     * Stops the timer-based flush and sends anything still in the queue.
     * Intended for process-teardown paths (RN `_shutdown` override). Swallows
     * errors so a failing final flush can't block the broader shutdown.
     *
     * If `timeoutMs` is provided, the final flush races against that budget so
     * a slow network/storage can't hold up shutdown indefinitely. Without it,
     * flush time is bounded only by `fetchRetryCount * (requestTimeout +
     * fetchRetryDelay)`, which can exceed the caller's shutdown SLA.
     */
    shutdown(timeoutMs?: number): Promise<void>;
    /**
     * Time-bounded flush for transient lifecycle events (e.g. RN
     * foreground→background) that must complete inside an OS-imposed window.
     * Unlike `shutdown`, this leaves the periodic flush timer in place so the
     * pipeline keeps draining if the process is resumed instead of suspended.
     *
     * Errors propagate so the host SDK can route them through its standard
     * lifecycle error handler (e.g. RN's `logFlushError`). If the timer wins
     * the race, a late rejection from the in-flight flush is silenced via a
     * no-op handler attached after the race settles, to avoid noisy
     * unhandled-rejection logs — the next regular flush cycle will retry.
     */
    flushWithTimeout(timeoutMs: number): Promise<void>;
    private _flushInBackground;
    private _clearFlushTimer;
}
//# sourceMappingURL=index.d.ts.map