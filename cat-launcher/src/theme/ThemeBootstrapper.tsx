import { ReactNode } from "react";

import { useTheme } from "./useTheme";
import { toastCL } from "@/lib/utils";

export interface ThemeBootstrapperProps {
  children: ReactNode;
}

export default function ThemeBootstrapper({
  children,
}: ThemeBootstrapperProps) {
  useTheme((error) => {
    toastCL("error", "Failed to load theme preference", error);
  });

  return <>{children}</>;
}
