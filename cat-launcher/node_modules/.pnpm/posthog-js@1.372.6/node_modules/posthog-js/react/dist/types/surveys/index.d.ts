import { RefCallback } from 'react';
import { SurveyPosition } from 'posthog-js';

interface UseThumbSurveyOptions {
    surveyId: string;
    displayPosition?: SurveyPosition;
    properties?: Record<string, any>;
    onResponse?: (response: 'up' | 'down') => void;
    disableAutoShownTracking?: boolean;
}
interface UseThumbSurveyResult {
    respond: (value: 'up' | 'down') => void;
    response: 'up' | 'down' | null;
    triggerRef: RefCallback<HTMLElement>;
    trackShown?: () => void;
}
declare function useThumbSurvey({ surveyId, displayPosition, properties, onResponse, disableAutoShownTracking, }: UseThumbSurveyOptions): UseThumbSurveyResult;

export { type UseThumbSurveyOptions, type UseThumbSurveyResult, useThumbSurvey };
