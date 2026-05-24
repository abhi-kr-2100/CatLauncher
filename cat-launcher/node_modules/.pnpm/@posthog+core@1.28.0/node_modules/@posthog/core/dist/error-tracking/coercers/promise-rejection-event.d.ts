import { CoercingContext, ErrorTrackingCoercer, ExceptionLike } from '../types';
type EventWithDetailReason = Event & {
    detail: {
        reason: unknown;
    };
};
export declare class PromiseRejectionEventCoercer implements ErrorTrackingCoercer<PromiseRejectionEvent | EventWithDetailReason> {
    match(err: unknown): err is PromiseRejectionEvent | EventWithDetailReason;
    private isCustomEventWrappingRejection;
    coerce(err: PromiseRejectionEvent | EventWithDetailReason, ctx: CoercingContext): ExceptionLike | undefined;
    private getUnhandledRejectionReason;
}
export {};
//# sourceMappingURL=promise-rejection-event.d.ts.map