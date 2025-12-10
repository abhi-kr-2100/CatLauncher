import { PostHog } from "posthog-js";

export enum Events {
  PageView = "page:view",
  ButtonClick = "button:click",
}

export const trackPageView = (posthog: PostHog, page: string) => {
  posthog.capture(Events.PageView, { page });
};

export const trackButtonClick = (
  posthog: PostHog,
  buttonName: string,
  properties: Record<string, any> = {}
) => {
  posthog.capture(Events.ButtonClick, { button_name: buttonName, ...properties });
};
