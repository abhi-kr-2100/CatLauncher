import { PropertyFilters, PropertyOperator } from '../posthog-surveys-types';
import type { Properties, SessionRecordingTriggerPropertyFilter } from '../types';
export declare function getPersonPropertiesHash(distinct_id: string, userPropertiesToSet?: Properties, userPropertiesToSetOnce?: Properties): string;
export declare const propertyComparisons: Record<PropertyOperator, (targets: string[], values: string[]) => boolean>;
/**
 * Evaluate trigger property filters (WHERE clauses) against event and person properties.
 * All filters must match (implicit AND). Returns true if no filters are present.
 */
export declare function matchTriggerPropertyFilters(filters: SessionRecordingTriggerPropertyFilter[] | undefined, eventProperties: Properties | undefined, personProperties: Properties | undefined): boolean;
export declare function matchPropertyFilters(propertyFilters: PropertyFilters | undefined, eventProperties: Properties | undefined): boolean;
