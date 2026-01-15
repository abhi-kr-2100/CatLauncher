import { useCallback } from "react";

import { toastCL } from "@/lib/utils";
import { ColorSettings } from "./components/ColorSettings";
import { FontSettings } from "./components/FontSettings";
import { MasterReset } from "./components/MasterReset";
import { SettingsPageFooter } from "./components/SettingsPageFooter";
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
    <div className="container mx-auto max-w-2xl px-4 pb-24">
      <form onSubmit={apply} className="space-y-8">
        <FontSettings control={form.control} />
        <ColorSettings control={form.control} />
        <MasterReset />

        <SettingsPageFooter
          isDirty={form.formState.isDirty}
          isUpdating={isUpdating}
          apply={apply}
          cancel={cancel}
          resetToDefault={resetToDefault}
        />
      </form>
    </div>
  );
}
