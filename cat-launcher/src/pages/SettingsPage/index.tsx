import { useCallback } from "react";

import { Button } from "@/components/ui/button";
import { toastCL } from "@/lib/utils";
import { FontSettings } from "./components/FontSettings";
import { useSettingsForm } from "./hooks";

export default function SettingsPage() {
  const onSettingsError = useCallback(
    (e: Error) => toastCL("error", "Failed to load settings.", e),
    [],
  );
  const onDefaultSettingsError = useCallback(
    (e: Error) =>
      toastCL("error", "Failed to load default settings.", e),
    [],
  );
  const onUpdateError = useCallback(
    (e: Error) => toastCL("error", "Failed to update settings.", e),
    [],
  );
  const onUpdateSuccess = useCallback(
    () => toastCL("success", "Settings updated successfully."),
    [],
  );

  const {
    form,
    isLoading,
    isUpdating,
    apply,
    cancel,
    resetToDefault,
  } = useSettingsForm({
    onSettingsError,
    onDefaultSettingsError,
    onUpdateError,
    onUpdateSuccess,
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        Loading...
      </div>
    );
  }

  return (
    <div className="container mx-auto py-10 max-w-2xl px-4">
      <h1 className="text-3xl font-bold mb-8">Settings</h1>
      <form onSubmit={apply} className="space-y-8">
        <FontSettings control={form.control} />

        <div className="flex justify-end gap-4">
          <Button
            type="button"
            variant="outline"
            onClick={resetToDefault}
          >
            Reset to Default
          </Button>
          <Button
            type="button"
            variant="ghost"
            onClick={cancel}
            disabled={!form.formState.isDirty}
          >
            Cancel
          </Button>
          <Button
            type="submit"
            disabled={!form.formState.isDirty || isUpdating}
          >
            Apply
          </Button>
        </div>
      </form>
    </div>
  );
}
