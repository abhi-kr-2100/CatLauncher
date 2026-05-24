"use strict";
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.PostHogExceptions = void 0;
exports.buildErrorPropertiesBuilder = buildErrorPropertiesBuilder;
var constants_1 = require("./constants");
var logger_1 = require("./utils/logger");
var property_utils_1 = require("./utils/property-utils");
var core_1 = require("@posthog/core");
var logger = (0, logger_1.createLogger)('[Error tracking]');
function buildErrorPropertiesBuilder() {
    return new core_1.ErrorTracking.ErrorPropertiesBuilder([
        new core_1.ErrorTracking.DOMExceptionCoercer(),
        new core_1.ErrorTracking.PromiseRejectionEventCoercer(),
        new core_1.ErrorTracking.ErrorEventCoercer(),
        new core_1.ErrorTracking.ErrorCoercer(),
        new core_1.ErrorTracking.EventCoercer(),
        new core_1.ErrorTracking.ObjectCoercer(),
        new core_1.ErrorTracking.StringCoercer(),
        new core_1.ErrorTracking.PrimitiveCoercer(),
    ], core_1.ErrorTracking.createDefaultStackParser());
}
var PostHogExceptions = /** @class */ (function () {
    function PostHogExceptions(instance) {
        var _a, _b;
        this._suppressionRules = [];
        this._errorPropertiesBuilder = buildErrorPropertiesBuilder();
        this._instance = instance;
        this._suppressionRules = (_b = (_a = this._instance.persistence) === null || _a === void 0 ? void 0 : _a.get_property(constants_1.ERROR_TRACKING_SUPPRESSION_RULES)) !== null && _b !== void 0 ? _b : [];
        this._exceptionStepsConfig = core_1.ErrorTracking.resolveExceptionStepsConfig(this._getExceptionStepsConfig());
        this._exceptionStepsBuffer = new core_1.ErrorTracking.ExceptionStepsBuffer(this._exceptionStepsConfig);
    }
    PostHogExceptions.prototype.onConfigChange = function () {
        this._exceptionStepsConfig = core_1.ErrorTracking.resolveExceptionStepsConfig(this._getExceptionStepsConfig());
        this._exceptionStepsBuffer.setConfig(this._exceptionStepsConfig);
    };
    PostHogExceptions.prototype.onRemoteConfig = function (response) {
        var _a;
        var _b, _c, _d;
        if (!('errorTracking' in response)) {
            return;
        }
        var suppressionRules = (_c = (_b = response.errorTracking) === null || _b === void 0 ? void 0 : _b.suppressionRules) !== null && _c !== void 0 ? _c : [];
        var captureExtensionExceptions = (_d = response.errorTracking) === null || _d === void 0 ? void 0 : _d.captureExtensionExceptions;
        // store this in-memory in case persistence is disabled
        this._suppressionRules = suppressionRules;
        if (this._instance.persistence) {
            this._instance.persistence.register((_a = {},
                _a[constants_1.ERROR_TRACKING_SUPPRESSION_RULES] = this._suppressionRules,
                _a[constants_1.ERROR_TRACKING_CAPTURE_EXTENSION_EXCEPTIONS] = captureExtensionExceptions,
                _a));
        }
    };
    Object.defineProperty(PostHogExceptions.prototype, "_captureExtensionExceptions", {
        get: function () {
            var _a;
            var enabled_server_side = !!this._instance.get_property(constants_1.ERROR_TRACKING_CAPTURE_EXTENSION_EXCEPTIONS);
            var enabled_client_side = this._instance.config.error_tracking.captureExtensionExceptions;
            return (_a = enabled_client_side !== null && enabled_client_side !== void 0 ? enabled_client_side : enabled_server_side) !== null && _a !== void 0 ? _a : false;
        },
        enumerable: false,
        configurable: true
    });
    PostHogExceptions.prototype.buildProperties = function (input, metadata) {
        return this._errorPropertiesBuilder.buildFromUnknown(input, {
            syntheticException: metadata === null || metadata === void 0 ? void 0 : metadata.syntheticException,
            mechanism: {
                handled: metadata === null || metadata === void 0 ? void 0 : metadata.handled,
            },
        });
    };
    PostHogExceptions.prototype.addExceptionStep = function (message, properties) {
        var _a;
        if (!this._exceptionStepsConfig.enabled) {
            return;
        }
        try {
            if (!(0, core_1.isString)(message) || message.trim().length === 0) {
                logger.warn('Ignoring exception step because message must be a non-empty string');
                return;
            }
            var userProperties = this._coerceExceptionStepProperties(properties);
            var _b = core_1.ErrorTracking.stripReservedExceptionStepFields(userProperties), sanitizedProperties = _b.sanitizedProperties, droppedKeys = _b.droppedKeys;
            if (droppedKeys.length > 0) {
                logger.warn('Ignoring reserved exception step fields', { droppedKeys: droppedKeys });
            }
            this._exceptionStepsBuffer.add(__assign((_a = {}, _a[core_1.ErrorTracking.EXCEPTION_STEP_INTERNAL_FIELDS.MESSAGE] = message, _a[core_1.ErrorTracking.EXCEPTION_STEP_INTERNAL_FIELDS.TIMESTAMP] = new Date().toISOString(), _a), sanitizedProperties));
        }
        catch (error) {
            logger.error('Failed to add exception step. Ignoring breadcrumb.', error);
        }
    };
    PostHogExceptions.prototype.sendExceptionEvent = function (properties) {
        try {
            var exceptionList = properties.$exception_list;
            if (this._isExceptionList(exceptionList)) {
                if (this._matchesSuppressionRule(exceptionList)) {
                    this._addDroppedExceptionStep('Exception dropped: matched a suppression rule');
                    logger.info('Skipping exception capture because a suppression rule matched');
                    return;
                }
                if (!this._captureExtensionExceptions && this._isExtensionException(exceptionList)) {
                    this._addDroppedExceptionStep('Exception dropped: thrown by a browser extension');
                    logger.info('Skipping exception capture because it was thrown by an extension');
                    return;
                }
                if (!this._instance.config.error_tracking.__capturePostHogExceptions &&
                    this._isPostHogException(exceptionList)) {
                    this._addDroppedExceptionStep('Exception dropped: thrown by the PostHog SDK');
                    logger.info('Skipping exception capture because it was thrown by the PostHog SDK');
                    return;
                }
            }
            var propertiesForExceptionCapture = this._exceptionStepsConfig.enabled && (0, core_1.isNullish)(properties.$exception_steps)
                ? this._addBufferedExceptionSteps(properties)
                : properties;
            try {
                var result = this._instance.capture('$exception', propertiesForExceptionCapture, {
                    _noTruncate: true,
                    _batchKey: 'exceptionEvent',
                    _originatedFromCaptureException: true,
                });
                if (result) {
                    this._exceptionStepsBuffer.clear();
                }
                return result;
            }
            catch (error) {
                logger.error('Failed to capture exception event. Dropping this exception.', error);
                this._exceptionStepsBuffer.clear();
                return;
            }
        }
        catch (error) {
            logger.error('Failed to process exception event. Ignoring this exception.', error);
            return;
        }
    };
    PostHogExceptions.prototype._addBufferedExceptionSteps = function (properties) {
        try {
            var exceptionSteps = this._exceptionStepsBuffer.getAttachable();
            if (exceptionSteps.length === 0) {
                return properties;
            }
            return __assign(__assign({}, properties), { $exception_steps: exceptionSteps });
        }
        catch (error) {
            logger.error('Failed to read buffered exception steps. Capturing exception without steps.', error);
            return properties;
        }
    };
    PostHogExceptions.prototype._addDroppedExceptionStep = function (message) {
        var _a;
        if (this._exceptionStepsConfig.enabled) {
            this._exceptionStepsBuffer.add((_a = {},
                _a[core_1.ErrorTracking.EXCEPTION_STEP_INTERNAL_FIELDS.MESSAGE] = message,
                _a[core_1.ErrorTracking.EXCEPTION_STEP_INTERNAL_FIELDS.TIMESTAMP] = new Date().toISOString(),
                _a));
        }
    };
    PostHogExceptions.prototype._coerceExceptionStepProperties = function (properties) {
        if (!(0, core_1.isObject)(properties)) {
            return {};
        }
        return __assign({}, properties);
    };
    PostHogExceptions.prototype._getExceptionStepsConfig = function () {
        var _a, _b;
        return (_b = (_a = this._instance.config.error_tracking) === null || _a === void 0 ? void 0 : _a.exception_steps) !== null && _b !== void 0 ? _b : {};
    };
    PostHogExceptions.prototype._matchesSuppressionRule = function (exceptionList) {
        if (exceptionList.length === 0) {
            return false;
        }
        var exceptionValues = exceptionList.reduce(function (acc, _a) {
            var type = _a.type, value = _a.value;
            if ((0, core_1.isString)(type) && type.length > 0) {
                acc['$exception_types'].push(type);
            }
            if ((0, core_1.isString)(value) && value.length > 0) {
                acc['$exception_values'].push(value);
            }
            return acc;
        }, {
            $exception_types: [],
            $exception_values: [],
        });
        return this._suppressionRules.some(function (rule) {
            var results = rule.values.map(function (v) {
                var _a;
                var compare = property_utils_1.propertyComparisons[v.operator];
                var targets = (0, core_1.isArray)(v.value) ? v.value : [v.value];
                var values = (_a = exceptionValues[v.key]) !== null && _a !== void 0 ? _a : [];
                return targets.length > 0 ? compare(targets, values) : false;
            });
            return rule.type === 'OR' ? results.some(Boolean) : results.every(Boolean);
        });
    };
    PostHogExceptions.prototype._isExtensionException = function (exceptionList) {
        var frames = exceptionList.flatMap(function (e) { var _a, _b; return (_b = (_a = e.stacktrace) === null || _a === void 0 ? void 0 : _a.frames) !== null && _b !== void 0 ? _b : []; });
        return frames.some(function (f) { return f.filename && f.filename.startsWith('chrome-extension://'); });
    };
    PostHogExceptions.prototype._isPostHogException = function (exceptionList) {
        var _a, _b, _c, _d;
        if (exceptionList.length > 0) {
            var exception = exceptionList[0];
            var frames_1 = (_b = (_a = exception.stacktrace) === null || _a === void 0 ? void 0 : _a.frames) !== null && _b !== void 0 ? _b : [];
            var lastFrame = frames_1[frames_1.length - 1];
            return (_d = (_c = lastFrame === null || lastFrame === void 0 ? void 0 : lastFrame.filename) === null || _c === void 0 ? void 0 : _c.includes('posthog.com/static')) !== null && _d !== void 0 ? _d : false;
        }
        return false;
    };
    PostHogExceptions.prototype._isExceptionList = function (candidate) {
        return !(0, core_1.isNullish)(candidate) && (0, core_1.isArray)(candidate);
    };
    return PostHogExceptions;
}());
exports.PostHogExceptions = PostHogExceptions;
//# sourceMappingURL=posthog-exceptions.js.map