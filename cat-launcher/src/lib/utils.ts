import clsx, { type ClassValue } from "clsx";
import { toast } from "sonner";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function toastCL(
  level: "error" | "warning",
  message: string,
  error: unknown,
) {
  toast[level](message);

  if (import.meta.env.DEV) {
    toast.info(JSON.stringify(error));
  }
}
