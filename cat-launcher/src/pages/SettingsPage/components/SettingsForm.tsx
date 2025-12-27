import { zodResolver } from "@hookform/resolvers/zod";
import { Controller, useForm } from "react-hook-form";
import * as z from "zod";

import { Button } from "@/components/ui/button";
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { toastCL, restartApp } from "@/lib/utils";
import { useApplySettings } from "../hooks/useApplySettings";
import { useSettings } from "../hooks/useSettings";
import FontSelector from "./FontSelector";
import ThemeSelector from "./ThemeSelector";

const settingsSchema = z.object({
  maxBackups: z.number().min(0).max(20),
  parallelRequests: z.number().min(1).max(16),
  fontLocation: z.string().nullable(),
  themeName: z.string().nullable(),
});

type SettingsFormValues = z.infer<typeof settingsSchema>;

export default function SettingsForm() {
  const {
    settings,
    themes,
    isLoading: dataLoading,
  } = useSettings((err) =>
    toastCL("error", "Failed to load settings", err),
  );
  const { applySettings, isPending } = useApplySettings(
    (err) => toastCL("error", "Failed to apply settings", err),
    () => {
      toastCL("success", "Settings applied successfully");
    },
  );

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = useForm<SettingsFormValues>({
    resolver: zodResolver(settingsSchema),
    values: settings
      ? {
          maxBackups: settings.max_backups,
          parallelRequests: settings.parallel_requests,
          fontLocation: settings.font ?? null,
          themeName: settings.themeName,
        }
      : undefined,
  });

  const onApply = (data: SettingsFormValues) => {
    const selectedThemeObj = themes.find(
      (t) => t.name === data.themeName,
    );

    applySettings({
      maxBackups: data.maxBackups,
      parallelRequests: data.parallelRequests,
      fontLocation: data.fontLocation ?? undefined,
      themeColors: selectedThemeObj?.colors,
    });
  };

  if (dataLoading) {
    return <div>Loading settings...</div>;
  }

  return (
    <form
      onSubmit={handleSubmit(onApply)}
      className="flex flex-col gap-10 p-4"
    >
      <div className="flex flex-row gap-6">
        <div className="flex-1">
          <Field>
            <FieldLabel htmlFor="max-backups">
              Max Backups (0-20)
            </FieldLabel>
            <FieldContent>
              <Input
                id="max-backups"
                type="number"
                {...register("maxBackups", { valueAsNumber: true })}
              />
              <FieldDescription>
                Reducing the number of automatic backups will cause
                old auto backups which are over the limit to be
                deleted. In particular, setting it to 0 will cause all
                automatic backups to be deleted. Manual backups will
                remain untouched.
              </FieldDescription>
              <FieldError errors={[errors.maxBackups]} />
            </FieldContent>
          </Field>
        </div>

        <div className="flex-1">
          <Field>
            <FieldLabel htmlFor="parallel-requests">
              Parallel Network Requests (1-16)
            </FieldLabel>
            <FieldContent>
              <Input
                id="parallel-requests"
                type="number"
                {...register("parallelRequests", {
                  valueAsNumber: true,
                })}
              />
              <FieldDescription>
                Higher values can increase download speed but may
                cause issues on slower connections or with some
                servers. Recommended: 4-8.
              </FieldDescription>
              <FieldError errors={[errors.parallelRequests]} />
            </FieldContent>
          </Field>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
        <Controller
          name="fontLocation"
          control={control}
          render={({ field }) => (
            <FontSelector
              selectedFont={field.value}
              onFontChange={field.onChange}
            />
          )}
        />

        <Controller
          name="themeName"
          control={control}
          render={({ field }) => (
            <ThemeSelector
              selectedTheme={field.value}
              onThemeChange={field.onChange}
            />
          )}
        />
      </div>

      <div className="text-sm text-muted-foreground">
        Some settings will be applied on restart.
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <Button type="submit" disabled={isPending} className="w-full">
          {isPending ? "Applying..." : "Apply Settings"}
        </Button>

        <Button
          type="button"
          variant="outline"
          onClick={() => restartApp()}
          className="w-full"
        >
          Restart Application
        </Button>
      </div>
    </form>
  );
}
