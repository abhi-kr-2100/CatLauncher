import { Moon, Sun } from "lucide-react";

import { Button } from "@/components/ui/button";
import { toastCL } from "@/lib/utils";
import { useTheme } from "./useTheme";

export default function ThemeToggle() {
  const { currentTheme, toggleTheme, isUpdating } = useTheme((error) => {
    toastCL("error", "Failed to update theme preference", error);
  });

  const Icon = currentTheme === "Dark" ? Sun : Moon;

  return (
    <Button
      type="button"
      variant="ghost"
      size="icon"
      onClick={toggleTheme}
      aria-label="Toggle theme"
      title="Toggle theme"
      disabled={isUpdating}
      className="text-muted-foreground hover:text-primary"
    >
      <Icon className="h-4 w-4" />
    </Button>
  );
}
