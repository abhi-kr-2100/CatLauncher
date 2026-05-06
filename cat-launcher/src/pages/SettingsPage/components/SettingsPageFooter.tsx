import { SyntheticEvent } from "react";

import { Button } from "@/components/ui/button";

/**
 * Props for the SettingsPageFooter component.
 */
interface SettingsPageFooterProps {
  /** Indicates if any form fields have been modified. */
  isDirty: boolean;
  /** Indicates if a settings update is currently in progress. */
  isUpdating: boolean;
  /** Function to apply the current changes. */
  apply: (e: SyntheticEvent) => void;
  /** Function to cancel changes and revert to the last saved state. */
  cancel: () => void;
  /** Function to reset all settings to their default values. */
  resetToDefault: () => void;
}

/**
 * The SettingsPageFooter component renders a fixed footer containing action buttons
 * for the settings form, such as "Reset to Default", "Cancel", and "Apply".
 *
 * @param props - The component props.
 * @returns A React component that renders the footer with settings actions.
 */
export function SettingsPageFooter({
  isDirty,
  isUpdating,
  apply,
  cancel,
  resetToDefault,
}: SettingsPageFooterProps) {
  return (
    <div className="fixed bottom-0 left-0 right-0 bg-background border-t border-border overflow-x-auto">
      <div className="container mx-auto max-w-2xl px-4 py-4">
        <div className="flex justify-end gap-4">
          <Button
            type="button"
            variant="outline"
            onClick={resetToDefault}
            disabled={isUpdating}
          >
            Reset to Default
          </Button>
          <Button
            type="button"
            variant="ghost"
            onClick={cancel}
            disabled={!isDirty}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            onClick={apply}
            disabled={!isDirty || isUpdating}
          >
            Apply
          </Button>
        </div>
      </div>
    </div>
  );
}
