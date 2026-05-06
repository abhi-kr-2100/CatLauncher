import { convertFileSrc } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import clsx, { type ClassValue } from "clsx";
import { toast } from "sonner";
import { twMerge } from "tailwind-merge";

import type { GameVariant } from "@/generated-types/GameVariant";

/**
 * Merges class names using `clsx` and `tailwind-merge`.
 *
 * @param inputs - A list of class values to be merged.
 * @returns A string of merged class names.
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Returns a human-friendly label for a given {@link GameVariant}.
 *
 * @param variant - The game variant to get the label for.
 * @returns The human-friendly label.
 */
export function getVariantLabel(variant: GameVariant): string {
  switch (variant) {
    case "DarkDaysAhead":
      return "Dark Days Ahead";
    case "BrightNights":
      return "Bright Nights";
    case "TheLastGeneration":
      return "The Last Generation";
  }
}

/**
 * Displays a toast message with a specific level and optional error details.
 * In development mode, if an error is provided, it also displays the error as an info toast.
 *
 * @param level - The severity level of the toast.
 * @param message - The primary message to display.
 * @param error - Optional error object for debugging purposes.
 */
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

/**
 * Opens a URL in the user's default browser.
 *
 * @param url - The URL to open.
 * @returns A promise that resolves when the URL is opened.
 */
export function openLink(url: string) {
  return openUrl(url);
}

/**
 * Copies the provided text to the system clipboard.
 *
 * @param text - The text to copy.
 * @returns A promise that resolves when the text is copied.
 */
export function copyToClipboard(text: string) {
  return navigator.clipboard.writeText(text);
}

/**
 * Sets up an event listener using a provided listen function and handler.
 * Manages the unlisten lifecycle and handles potential errors.
 *
 * @param listenFn - A function that sets up the listener and returns an unlisten function.
 * @param handler - The function to be called when the event occurs.
 * @param listenErrorMessage - The error message to display if the listener fails to set up.
 * @param onError - Optional callback for error handling.
 * @returns A cleanup function to unlisten from the event.
 */
export function setupEventListener<T>(
  listenFn: (handler: (payload: T) => void) => Promise<() => void>,
  handler: (payload: T) => void,
  listenErrorMessage: string,
  onError?: (error: unknown) => void,
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
        onError?.(error);
      }
    });

  return () => {
    cancelled = true;
    unlisten?.();
  };
}

/**
 * Generates a random integer between 0 and n - 1.
 *
 * @param n - The upper bound (exclusive).
 * @returns A random integer.
 */
export function randomInt(n: number): number {
  return Math.floor(Math.random() * n);
}

/**
 * Sets an interval that executes the callback immediately and then at the specified timeout.
 *
 * @param callback - The function to execute.
 * @param timeout - The interval timeout in milliseconds.
 * @returns The interval ID.
 */
export function setImmediateInterval(
  callback: () => void,
  timeout?: number,
) {
  callback();

  return setInterval(callback, timeout);
}

/**
 * Converts a snake_case or messy string into a human-friendly format by replacing underscores with spaces and collapsing extra whitespace.
 *
 * @param text - The string to format.
 * @returns The human-friendly string.
 */
export function getHumanFriendlyText(text: string): string {
  return text.replace(/_/g, " ").replace(/\s+/g, " ").trim();
}

/**
 * Formats a number of bytes into a more readable format (e.g., KB, MB, GB).
 *
 * @param bytes - The number of bytes to format.
 * @returns A tuple containing the formatted number and its unit.
 */
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
 *
 * @param path - The file path of the font.
 * @returns A CSS-safe font family name.
 */
function getFontFamily(path: string) {
  let hash = 0;
  for (let i = 0; i < path.length; i++) {
    hash = (hash << 5) - hash + path.charCodeAt(i);
    hash |= 0;
  }
  return `fp-${Math.abs(hash)}`;
}

/**
 * Ensures that a font at the specified path is loaded into the document.
 * If the font is already loaded, it returns the existing font family name.
 *
 * @param path - The file path of the font to load.
 * @returns A promise that resolves to the font family name.
 */
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
