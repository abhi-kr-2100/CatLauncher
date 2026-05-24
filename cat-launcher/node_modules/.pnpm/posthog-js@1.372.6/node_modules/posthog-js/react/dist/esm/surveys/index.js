import { createContext, useContext, useState, useMemo, useRef, useCallback, useEffect } from 'react';
import { SurveyPosition, SurveyEventName, SurveyEventProperties, DisplaySurveyType } from 'posthog-js';

/******************************************************************************
Copyright (c) Microsoft Corporation.

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
***************************************************************************** */

var __assign = function() {
    __assign = Object.assign || function __assign(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};

typeof SuppressedError === "function" ? SuppressedError : function (error, suppressed, message) {
    var e = new Error(message);
    return e.name = "SuppressedError", e.error = error, e.suppressed = suppressed, e;
};

var defaultPostHogInstance;
function getDefaultPostHogInstance() {
    return defaultPostHogInstance;
}

var PostHogContext = createContext({
    get client() {
        return getDefaultPostHogInstance();
    },
    bootstrap: undefined,
});

var usePostHog = function () {
    var client = useContext(PostHogContext).client;
    return client;
};

var TRIGGER_ATTR = 'data-ph-thumb-survey-trigger';
function useThumbSurvey(_a) {
    var surveyId = _a.surveyId, _b = _a.displayPosition, displayPosition = _b === void 0 ? SurveyPosition.NextToTrigger : _b, properties = _a.properties, onResponse = _a.onResponse, disableAutoShownTracking = _a.disableAutoShownTracking;
    var posthog = usePostHog();
    var _c = useState(null), responded = _c[0], setResponded = _c[1];
    var instanceId = useState(function () { return Math.random().toString(36).slice(2, 9); })[0];
    var triggerValue = useMemo(function () { return "".concat(surveyId, "-").concat(instanceId); }, [surveyId, instanceId]);
    var elementRef = useRef(null);
    var triggerRef = useCallback(function (el) {
        if (elementRef.current) {
            elementRef.current.removeAttribute(TRIGGER_ATTR);
        }
        elementRef.current = el;
        if (el) {
            el.setAttribute(TRIGGER_ATTR, triggerValue);
        }
    }, [triggerValue]);
    var shownRef = useRef(false);
    var respondedRef = useRef(false);
    var trackShown = useCallback(function () {
        var _a;
        var _b;
        if (shownRef.current || !posthog)
            return;
        shownRef.current = true;
        posthog.capture(SurveyEventName.SHOWN, __assign((_a = {}, _a[SurveyEventProperties.SURVEY_ID] = surveyId, _a.sessionRecordingUrl = (_b = posthog.get_session_replay_url) === null || _b === void 0 ? void 0 : _b.call(posthog), _a), properties));
    }, [posthog, surveyId, properties]);
    useEffect(function () {
        if (!disableAutoShownTracking) {
            trackShown();
        }
    }, [trackShown, disableAutoShownTracking]);
    var respond = useCallback(function (value) {
        if (!(posthog === null || posthog === void 0 ? void 0 : posthog.surveys) || respondedRef.current)
            return;
        respondedRef.current = true;
        setResponded(value);
        onResponse === null || onResponse === void 0 ? void 0 : onResponse(value);
        posthog.surveys.displaySurvey(surveyId, {
            displayType: DisplaySurveyType.Popover,
            ignoreConditions: true,
            ignoreDelay: true,
            properties: properties,
            initialResponses: { 0: value === 'up' ? 1 : 2 },
            position: displayPosition,
            selector: "[".concat(TRIGGER_ATTR, "=\"").concat(triggerValue, "\"]"),
            skipShownEvent: true,
        });
    }, [posthog, surveyId, displayPosition, properties, onResponse, triggerValue]);
    return __assign({ respond: respond, response: responded, triggerRef: triggerRef }, (disableAutoShownTracking && { trackShown: trackShown }));
}

export { useThumbSurvey };
//# sourceMappingURL=index.js.map
