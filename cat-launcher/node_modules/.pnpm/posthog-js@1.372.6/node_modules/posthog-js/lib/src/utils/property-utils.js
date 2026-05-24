"use strict";
var __read = (this && this.__read) || function (o, n) {
    var m = typeof Symbol === "function" && o[Symbol.iterator];
    if (!m) return o;
    var i = m.call(o), r, ar = [], e;
    try {
        while ((n === void 0 || n-- > 0) && !(r = i.next()).done) ar.push(r.value);
    }
    catch (error) { e = { error: error }; }
    finally {
        try {
            if (r && !r.done && (m = i["return"])) m.call(i);
        }
        finally { if (e) throw e.error; }
    }
    return ar;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.propertyComparisons = void 0;
exports.getPersonPropertiesHash = getPersonPropertiesHash;
exports.matchTriggerPropertyFilters = matchTriggerPropertyFilters;
exports.matchPropertyFilters = matchPropertyFilters;
var core_1 = require("@posthog/core");
var request_1 = require("../request");
var regex_utils_1 = require("./regex-utils");
function getPersonPropertiesHash(distinct_id, userPropertiesToSet, userPropertiesToSetOnce) {
    return (0, request_1.jsonStringify)({ distinct_id: distinct_id, userPropertiesToSet: userPropertiesToSet, userPropertiesToSetOnce: userPropertiesToSetOnce });
}
exports.propertyComparisons = {
    exact: function (targets, values) { return values.some(function (value) { return targets.some(function (target) { return value === target; }); }); },
    is_not: function (targets, values) { return values.every(function (value) { return targets.every(function (target) { return value !== target; }); }); },
    regex: function (targets, values) { return values.some(function (value) { return targets.some(function (target) { return (0, regex_utils_1.isMatchingRegex)(value, target); }); }); },
    not_regex: function (targets, values) { return values.every(function (value) { return targets.every(function (target) { return !(0, regex_utils_1.isMatchingRegex)(value, target); }); }); },
    icontains: function (targets, values) {
        return values.map(toLowerCase).some(function (value) { return targets.map(toLowerCase).some(function (target) { return value.includes(target); }); });
    },
    not_icontains: function (targets, values) {
        return values.map(toLowerCase).every(function (value) { return targets.map(toLowerCase).every(function (target) { return !value.includes(target); }); });
    },
    gt: function (targets, values) {
        return values.some(function (value) {
            var numValue = parseFloat(value);
            return !isNaN(numValue) && targets.some(function (t) { return numValue > parseFloat(t); });
        });
    },
    lt: function (targets, values) {
        return values.some(function (value) {
            var numValue = parseFloat(value);
            return !isNaN(numValue) && targets.some(function (t) { return numValue < parseFloat(t); });
        });
    },
};
var toLowerCase = function (v) { return v.toLowerCase(); };
// Operators whose semantics mean "property is not X". When the property being
// filtered on is missing or null, these match — absence of the property
// satisfies a "not equal to X" check. This aligns with how PostHog's feature
// flag matchers (posthog/queries/base.py, rust/feature-flags) treat missing
// properties for negative operators.
var NEGATIVE_OPERATORS = new Set(['is_not', 'not_icontains', 'not_regex']);
/**
 * Evaluate trigger property filters (WHERE clauses) against event and person properties.
 * All filters must match (implicit AND). Returns true if no filters are present.
 */
function matchTriggerPropertyFilters(filters, eventProperties, personProperties) {
    if (!filters || filters.length === 0) {
        return true;
    }
    return filters.every(function (filter) {
        var source = filter.type === 'person' ? personProperties : eventProperties;
        var propertyValue = source === null || source === void 0 ? void 0 : source[filter.key];
        var operator = filter.operator || 'exact';
        // Missing or null property: for negative operators, absence counts as a
        // match (nothing can't equal EU, so "is_not EU" is satisfied). For
        // positive operators, we can't confirm a match without a value.
        if ((0, core_1.isUndefined)(propertyValue) || (0, core_1.isNull)(propertyValue)) {
            return NEGATIVE_OPERATORS.has(operator);
        }
        var comparisonFunction = exports.propertyComparisons[operator];
        if (!comparisonFunction) {
            return false;
        }
        if ((0, core_1.isUndefined)(filter.value) || (0, core_1.isNull)(filter.value)) {
            return false;
        }
        // Normalize filter value and property value to string arrays for comparison
        var targetValues = (0, core_1.isArray)(filter.value) ? filter.value.map(String) : [String(filter.value)];
        var actualValues = (0, core_1.isArray)(propertyValue) ? propertyValue.map(String) : [String(propertyValue)];
        return comparisonFunction(targetValues, actualValues);
    });
}
function matchPropertyFilters(propertyFilters, eventProperties) {
    // if there are no property filters, it means we're only matching on event name
    if (!propertyFilters) {
        return true;
    }
    return Object.entries(propertyFilters).every(function (_a) {
        var _b = __read(_a, 2), propertyName = _b[0], filter = _b[1];
        var eventPropertyValue = eventProperties === null || eventProperties === void 0 ? void 0 : eventProperties[propertyName];
        if ((0, core_1.isUndefined)(eventPropertyValue) || (0, core_1.isNull)(eventPropertyValue)) {
            return false;
        }
        // convert event property to string array for comparison
        var eventValues = [String(eventPropertyValue)];
        var comparisonFunction = exports.propertyComparisons[filter.operator];
        if (!comparisonFunction) {
            return false;
        }
        return comparisonFunction(filter.values, eventValues);
    });
}
//# sourceMappingURL=property-utils.js.map