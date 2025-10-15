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
