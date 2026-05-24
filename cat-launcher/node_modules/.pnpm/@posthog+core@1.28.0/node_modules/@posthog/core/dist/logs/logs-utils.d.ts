import type { CaptureLogOptions, LogAttributeValue, LogSeverityLevel, OtlpAnyValue, OtlpKeyValue, OtlpLogRecord, OtlpLogsPayload, OtlpSeverityText } from '@posthog/types';
import type { LogSdkContext } from './types';
export declare function getOtlpSeverityText(level: LogSeverityLevel): OtlpSeverityText;
export declare function getOtlpSeverityNumber(level: LogSeverityLevel): number;
export declare function toOtlpAnyValue(value: LogAttributeValue): OtlpAnyValue;
export declare function toOtlpKeyValueList(attrs: Record<string, LogAttributeValue>): OtlpKeyValue[];
/**
 * Builds a single OTLP log record.
 *
 * Auto-attribute population is shape-driven: any field present on `sdkContext`
 * is emitted as the corresponding attribute. Each SDK populates only the
 * fields that apply to it (browser fills `currentUrl`; mobile fills
 * `screenName` / `appState`), so a missing field never adds a stray attribute.
 *
 * User-provided `options.attributes` always wins on conflicts.
 */
export declare function buildOtlpLogRecord(options: CaptureLogOptions, sdkContext: LogSdkContext): OtlpLogRecord;
/**
 * Wraps a list of records in the OTLP `resourceLogs` envelope.
 *
 * `scopeName` is the SDK package name (`posthog-js`, `posthog-react-native`,
 * etc.). `scopeVersion` is the SDK semver. The server combines them into a
 * single `instrumentation_scope` field (`{name}@{version}`) used for
 * SDK-version-level attribution in queries and dashboards.
 */
export declare function buildOtlpLogsPayload(logRecords: OtlpLogRecord[], resourceAttributes: Record<string, LogAttributeValue>, scopeName: string, scopeVersion: string): OtlpLogsPayload;
//# sourceMappingURL=logs-utils.d.ts.map