import { SyntheticEvent } from "react";

import { Button } from "@/components/ui/button";

interface SettingsPageFooterProps {
  isDirty: boolean;
  isUpdating: boolean;
  apply: (e: SyntheticEvent) => void;
  cancel: () => void;
  resetToDefault: () => void;
}

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
