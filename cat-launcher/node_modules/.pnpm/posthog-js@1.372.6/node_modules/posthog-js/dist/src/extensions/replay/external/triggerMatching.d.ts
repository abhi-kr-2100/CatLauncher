import { PostHog } from '../../../posthog-core';
import { FlagVariant, RemoteConfig, SessionRecordingPersistedConfig, SessionRecordingUrlTrigger } from '../../../types';
export declare const DISABLED = "disabled";
export declare const SAMPLED = "sampled";
export declare const ACTIVE = "active";
export declare const BUFFERING = "buffering";
export declare const PAUSED = "paused";
export declare const LAZY_LOADING = "lazy_loading";
export declare const AWAITING_CONFIG = "awaiting_config";
export declare const MISSING_CONFIG = "missing_config";
export declare const RRWEB_ERROR = "rrweb_error";
export declare const TRIGGER_ACTIVATED: string;
export declare const TRIGGER_PENDING: string;
export declare const TRIGGER_DISABLED: string;
export interface RecordingTriggersStatus {
    get receivedFlags(): boolean;
    get isRecordingEnabled(): false | true | undefined;
    get isSampled(): false | true | null;
    get rrwebError(): boolean;
    get urlTriggerMatching(): URLTriggerMatching;
    get eventTriggerMatching(): EventTriggerMatching;
    get linkedFlagMatching(): LinkedFlagMatching;
    get sessionId(): string;
}
/**
 * Extended interface for V2 trigger groups
 */
export interface RecordingTriggersStatusV2 extends RecordingTriggersStatus {
    get triggerGroupMatchers(): TriggerGroupMatching[];
    get triggerGroupSamplingResults(): Map<string, boolean>;
    get minimumDuration(): number | null;
}
export type TriggerType = 'url' | 'event';
declare const triggerStatuses: readonly [string, string, string];
export type TriggerStatus = (typeof triggerStatuses)[number];
/**
 * Session recording starts in buffering mode while waiting for "flags response".
 * Once the response is received, it might be disabled, active or sampled.
 * When "sampled" that means a sample rate is set, and the last time the session ID rotated
 * the sample rate determined this session should be sent to the server.
 */
declare const sessionRecordingStatuses: readonly ["disabled", "sampled", "active", "buffering", "paused", "lazy_loading", "awaiting_config", "missing_config", "rrweb_error"];
export type SessionRecordingStatus = (typeof sessionRecordingStatuses)[number];
type ReplayConfigType = RemoteConfig | SessionRecordingPersistedConfig;
type TriggerMatchingConfig = Pick<SessionRecordingPersistedConfig, 'urlTriggers' | 'urlBlocklist' | 'eventTriggers' | 'linkedFlag'>;
export interface TriggerStatusMatching {
    triggerStatus(sessionId: string): TriggerStatus;
    stop(): void;
}
export declare class OrTriggerMatching implements TriggerStatusMatching {
    private readonly _matchers;
    constructor(_matchers: TriggerStatusMatching[]);
    triggerStatus(sessionId: string): TriggerStatus;
    stop(): void;
}
export declare class AndTriggerMatching implements TriggerStatusMatching {
    private readonly _matchers;
    constructor(_matchers: TriggerStatusMatching[]);
    triggerStatus(sessionId: string): TriggerStatus;
    stop(): void;
}
export declare class PendingTriggerMatching implements TriggerStatusMatching {
    triggerStatus(): TriggerStatus;
    stop(): void;
}
export declare class AlwaysActivatedTriggerMatching implements TriggerStatusMatching {
    triggerStatus(): TriggerStatus;
    stop(): void;
}
export declare class URLTriggerMatching implements TriggerStatusMatching {
    private readonly _instance;
    _urlTriggers: SessionRecordingUrlTrigger[];
    _urlBlocklist: SessionRecordingUrlTrigger[];
    private _compiledTriggerRegexes;
    private _compiledBlocklistRegexes;
    private _lastCheckedUrl;
    private _groupId?;
    urlBlocked: boolean;
    constructor(_instance: PostHog, groupId?: string);
    onConfig(config: ReplayConfigType | TriggerMatchingConfig): void;
    /**
     * Compiles and caches RegExp objects from URL triggers and blocklist.
     * This prevents recreating RegExp objects on every rrweb event
     */
    private _compileRegexCache;
    /**
     * @deprecated Use onConfig instead
     */
    onRemoteConfig(response: RemoteConfig): void;
    private _urlTriggerStatus;
    triggerStatus(sessionId: string): TriggerStatus;
    /**
     * Check URL blocklist and pause/resume recording accordingly
     * This is separate from trigger checking and is used by both V1 and V2
     *
     * Performance optimization: Only checks when URL changes to avoid redundant regex matching
     */
    checkUrlBlocklist(onPause: () => void, onResume: () => void): void;
    checkUrlTriggerConditions(onPause: () => void, onResume: () => void, onActivate: (triggerType: TriggerType, matchDetail?: string) => void, sessionId: string): void;
    stop(): void;
}
export declare class LinkedFlagMatching implements TriggerStatusMatching {
    private readonly _instance;
    linkedFlag: string | FlagVariant | null;
    linkedFlagSeen: boolean;
    private _flagListenerCleanup;
    constructor(_instance: PostHog);
    triggerStatus(): TriggerStatus;
    onConfig(config: ReplayConfigType | TriggerMatchingConfig, onStarted: (flag: string, variant: string | null) => void): void;
    /**
     * @deprecated Use onConfig instead
     */
    onRemoteConfig(response: RemoteConfig, onStarted: (flag: string, variant: string | null) => void): void;
    stop(): void;
}
export declare class EventTriggerMatching implements TriggerStatusMatching {
    private readonly _instance;
    _eventTriggers: string[];
    private _groupId?;
    constructor(_instance: PostHog, groupId?: string);
    onConfig(config: ReplayConfigType | TriggerMatchingConfig): void;
    /**
     * @deprecated Use onConfig instead
     */
    onRemoteConfig(response: RemoteConfig): void;
    private _eventTriggerStatus;
    triggerStatus(sessionId: string): TriggerStatus;
    checkEventTriggerConditions(eventName: string, onActivate: (triggerType: TriggerType, matchDetail?: string) => void, sessionId: string): void;
    stop(): void;
}
/**
 * V2 Trigger Group Matching - manages a single trigger group with its own conditions
 */
export declare class TriggerGroupMatching implements TriggerStatusMatching {
    private readonly _instance;
    private _urlTriggerMatching;
    private _eventTriggerMatching;
    private _linkedFlagMatching;
    private _combinedMatching;
    readonly group: import('../../../types').SessionRecordingTriggerGroup;
    constructor(_instance: PostHog, group: import('../../../types').SessionRecordingTriggerGroup, onFlagStarted: (flag: string, variant: string | null) => void);
    triggerStatus(sessionId: string): TriggerStatus;
    checkEventTriggerConditions(eventName: string, onActivate: (triggerType: TriggerType, matchDetail?: string) => void, sessionId: string): void;
    checkUrlTriggerConditions(onPause: () => void, onResume: () => void, onActivate: (triggerType: TriggerType, matchDetail?: string) => void, sessionId: string): void;
    /**
     * V2: Activate this group's trigger and persist to group-specific key
     * This prevents cross-group contamination and survives page reloads
     */
    activateTrigger(triggerType: TriggerType, sessionId: string): void;
    stop(): void;
}
export declare function nullMatchSessionRecordingStatus(triggersStatus: RecordingTriggersStatus): SessionRecordingStatus;
export declare function anyMatchSessionRecordingStatus(triggersStatus: RecordingTriggersStatus): SessionRecordingStatus;
export declare function allMatchSessionRecordingStatus(triggersStatus: RecordingTriggersStatus): SessionRecordingStatus;
/**
 * V2 Trigger Groups Status Matcher - implements union behavior:
 * 1. Evaluate ALL trigger groups
 * 2. For each matching group, check if its sample rate hits
 * 3. If ANY group's sample rate hits → record session
 */
export declare function triggerGroupsMatchSessionRecordingStatus(triggersStatus: RecordingTriggersStatusV2): SessionRecordingStatus;
export {};
