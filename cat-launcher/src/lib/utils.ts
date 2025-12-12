import clsx, { type ClassValue } from "clsx";
import { toast } from "sonner";
import { twMerge } from "tailwind-merge";
import { openUrl } from "@tauri-apps/plugin-opener";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function toastCL(
  level: "error" | "warning" | "info" | "success",
  message: string,
  error?: unknown,
) {
  toast[level](message);

  if (import.meta.env.DEV && error !== undefined) {
    toast.info(JSON.stringify(error));
  }
}

export function openLink(url: string) {
  return openUrl(url);
}

export function copyToClipboard(text: string) {
  return navigator.clipboard.writeText(text);
}

export function setupEventListener<T>(
  listenFn: (handler: (payload: T) => void) => Promise<() => void>,
  handler: (payload: T) => void,
  listenErrorMessage: string,
) {
  let unlisten: (() => void) | undefined;
  let cancelled = false;

  listenFn(handler)
    .then((unlistenFn) => {
      if (cancelled) {
        unlistenFn();
      } else {
        unlisten = unlistenFn;
      }
    })
    .catch((error) => {
      if (!cancelled) {
        toastCL("error", listenErrorMessage, error);
      }
    });

  return () => {
    cancelled = true;
    unlisten?.();
  };
}

export function randomInt(n: number): number {
  return Math.floor(Math.random() * n);
}

export function setImmediateInterval(callback: () => void, timeout?: number) {
  callback();

  return setInterval(callback, timeout);
}

export function getHumanFriendlyText(text: string): string {
  return text.replace(/_/g, " ").replace(/\s+/g, " ").trim();
}
