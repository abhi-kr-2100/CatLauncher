/**
 * @posthog/types - Type definitions for the PostHog JavaScript SDK
 *
 * This package provides TypeScript type definitions for the PostHog SDK,
 * allowing you to type the PostHog instance and its configuration options.
 */
export type { PostHog } from './posthog';
export type { Property, Properties, JsonType, JsonRecord } from './common';
export type { KnownEventName, KnownUnsafeEditableEvent, EventName, CaptureResult, CaptureOptions, BeforeSendFn, } from './capture';
export type { FeatureFlagsCallback, FeatureFlagDetail, FeatureFlagMetadata, EvaluationReason, FeatureFlagResult, FeatureFlagOptions, RemoteConfigFeatureFlagCallback, EarlyAccessFeature, EarlyAccessFeatureStage, EarlyAccessFeatureCallback, EarlyAccessFeatureResponse, FeatureFlagOverrides, FeatureFlagPayloadOverrides, FeatureFlagOverrideOptions, OverrideFeatureFlagsOptions, } from './feature-flags';
export type { Headers, RequestResponse, RequestCallback } from './request';
export type { SessionRecordingCanvasOptions, InitiatorType, NetworkRequest, CapturedNetworkRequest, SessionIdChangedCallback, SeverityLevel, } from './session-recording';
export type { AutocaptureCompatibleElement, DomAutocaptureEvents, AutocaptureConfig, RageclickConfig, BootstrapConfig, SupportedWebVitalsMetrics, PerformanceCaptureConfig, DeadClickCandidate, ExceptionAutoCaptureConfig, ExceptionStepsConfig, DeadClicksAutoCaptureConfig, HeatmapConfig, ConfigDefaults, ExternalIntegrationKind, ErrorTrackingOptions, MaskInputOptions, SlimDOMOptions, SessionRecordingOptions, RequestQueueConfig, LogCaptureOptions, PostHogConfig, } from './posthog-config';
export type { SegmentUser, SegmentAnalytics, SegmentPlugin, SegmentContext, SegmentFunction } from './segment';
export type { SurveyRenderReason } from './survey';
export type { ToolbarParams, ToolbarUserIntent, ToolbarSource, ToolbarVersion } from './toolbar';
export type { TreeShakeableConfig, TreeShakeable } from './tree-shakeable';
export type { LogSeverityLevel, OtlpSeverityText, OtlpSeverityEntry, LogAttributeValue, LogAttributes, CaptureLogOptions, Logger, OtlpAnyValue, OtlpKeyValue, OtlpLogRecord, OtlpLogsPayload, } from './capture-log';
//# sourceMappingURL=index.d.ts.map