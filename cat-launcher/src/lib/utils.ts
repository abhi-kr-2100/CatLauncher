import { convertFileSrc } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import clsx, { type ClassValue } from "clsx";
import { toast } from "sonner";
import { twMerge } from "tailwind-merge";

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

export function setImmediateInterval(
  callback: () => void,
  timeout?: number,
) {
  callback();

  return setInterval(callback, timeout);
}

export function getHumanFriendlyText(text: string): string {
  return text.replace(/_/g, " ").replace(/\s+/g, " ").trim();
}

export function formatBytes(bytes: number): [number, string] {
  if (bytes == 0) {
    return [0, "B"];
  }

  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.min(
    sizes.length - 1,
    Math.floor(Math.log(bytes) / Math.log(k)),
  );

  const number = parseFloat((bytes / Math.pow(k, i)).toFixed(2));
  const size = sizes[i];

  return [number, size];
}

const loadedFonts = new Set<string>();

/**
 * Generates a stable, CSS-safe font family name for a given font path.
 */
function getFontFamily(path: string) {
  let hash = 0;
  for (let i = 0; i < path.length; i++) {
    hash = (hash << 5) - hash + path.charCodeAt(i);
    hash |= 0;
  }
  return `fp-${Math.abs(hash)}`;
}

export async function ensureFontLoaded(path: string) {
  const family = getFontFamily(path);
  if (loadedFonts.has(family)) {
    return family;
  }

  const src = convertFileSrc(path);
  const fontFace = new FontFace(family, `url("${src}")`);
  const loaded = await fontFace.load();
  document.fonts.add(loaded);
  loadedFonts.add(family);

  return family;
}
