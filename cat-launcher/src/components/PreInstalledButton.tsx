import { Button } from "./ui/button";

/**
 * A button component that represents a pre-installed state.
 * It is disabled by default to indicate that no further action is required for installation.
 *
 * @returns A React element representing the pre-installed button.
 *
 * @public
 */
export function PreInstalledButton() {
  return (
    <Button className="w-full" disabled>
      Pre-Installed
    </Button>
  );
}
