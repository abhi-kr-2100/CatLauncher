import { Extension } from './extensions/types';
import { PostHog } from './posthog-core';
import { CaptureResult, Properties, RemoteConfig } from './types';
import { ErrorTracking } from '@posthog/core';
export declare function buildErrorPropertiesBuilder(): ErrorTracking.ErrorPropertiesBuilder;
export declare class PostHogExceptions implements Extension {
    private readonly _instance;
    private _suppressionRules;
    private _errorPropertiesBuilder;
    private _exceptionStepsBuffer;
    private _exceptionStepsConfig;
    constructor(instance: PostHog);
    onConfigChange(): void;
    onRemoteConfig(response: RemoteConfig): void;
    private get _captureExtensionExceptions();
    buildProperties(input: unknown, metadata?: {
        handled?: boolean;
        syntheticException?: Error;
    }): ErrorTracking.ErrorProperties;
    addExceptionStep(message: string, properties?: Properties): void;
    sendExceptionEvent(properties: Properties): CaptureResult | undefined;
    private _addBufferedExceptionSteps;
    private _addDroppedExceptionStep;
    private _coerceExceptionStepProperties;
    private _getExceptionStepsConfig;
    private _matchesSuppressionRule;
    private _isExtensionException;
    private _isPostHogException;
    private _isExceptionList;
}
