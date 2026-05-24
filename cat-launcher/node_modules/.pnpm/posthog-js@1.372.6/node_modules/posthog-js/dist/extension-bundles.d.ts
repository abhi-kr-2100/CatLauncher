import { BundleTypes } from './module.slim';

declare const AllExtensions: typeof BundleTypes.AllExtensions;
declare const FeatureFlagsExtensions: typeof BundleTypes.FeatureFlagsExtensions;
declare const SessionReplayExtensions: typeof BundleTypes.SessionReplayExtensions;
declare const AnalyticsExtensions: typeof BundleTypes.AnalyticsExtensions;
declare const ErrorTrackingExtensions: typeof BundleTypes.ErrorTrackingExtensions;
declare const ProductToursExtensions: typeof BundleTypes.ProductToursExtensions;
declare const SiteAppsExtensions: typeof BundleTypes.SiteAppsExtensions;
declare const SurveysExtensions: typeof BundleTypes.SurveysExtensions;
declare const TracingExtensions: typeof BundleTypes.TracingExtensions;
declare const ToolbarExtensions: typeof BundleTypes.ToolbarExtensions;
declare const ExperimentsExtensions: typeof BundleTypes.ExperimentsExtensions;
declare const ConversationsExtensions: typeof BundleTypes.ConversationsExtensions;
declare const LogsExtensions: typeof BundleTypes.LogsExtensions;

export { AllExtensions, AnalyticsExtensions, ConversationsExtensions, ErrorTrackingExtensions, ExperimentsExtensions, FeatureFlagsExtensions, LogsExtensions, ProductToursExtensions, SessionReplayExtensions, SiteAppsExtensions, SurveysExtensions, ToolbarExtensions, TracingExtensions };
