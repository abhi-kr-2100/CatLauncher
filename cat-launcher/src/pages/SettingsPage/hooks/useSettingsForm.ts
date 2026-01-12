import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef } from "react";
import { useForm } from "react-hook-form";

import type { Settings } from "@/generated-types/Settings";
import {
  getDefaultSettings,
  getSettings,
  updateSettings,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export interface UseSettingsFormProps {
  onSettingsError?: (error: Error) => void;
  onDefaultSettingsError?: (error: Error) => void;
  onUpdateError?: (error: Error) => void;
  onUpdateSuccess?: () => void;
}

export function useSettingsForm({
  onSettingsError,
  onDefaultSettingsError,
  onUpdateError,
  onUpdateSuccess,
}: UseSettingsFormProps = {}) {
  const queryClient = useQueryClient();

  const onSettingsErrorRef = useRef(onSettingsError);
  const onDefaultSettingsErrorRef = useRef(onDefaultSettingsError);
  const onUpdateErrorRef = useRef(onUpdateError);
  const onUpdateSuccessRef = useRef(onUpdateSuccess);

  useEffect(() => {
    onSettingsErrorRef.current = onSettingsError;
    onDefaultSettingsErrorRef.current = onDefaultSettingsError;
    onUpdateErrorRef.current = onUpdateError;
    onUpdateSuccessRef.current = onUpdateSuccess;
  }, [
    onSettingsError,
    onDefaultSettingsError,
    onUpdateError,
    onUpdateSuccess,
  ]);

  const {
    data: settings,
    isLoading: isLoadingSettings,
    error: settingsError,
  } = useQuery({
    queryKey: queryKeys.settings(),
    queryFn: getSettings,
  });

  const {
    data: defaultSettings,
    isLoading: isLoadingDefaultSettings,
    error: defaultSettingsError,
  } = useQuery({
    queryKey: queryKeys.defaultSettings(),
    queryFn: getDefaultSettings,
  });

  useEffect(() => {
    if (settingsError && onSettingsErrorRef.current) {
      onSettingsErrorRef.current(settingsError as Error);
    }
  }, [settingsError]);

  useEffect(() => {
    if (defaultSettingsError && onDefaultSettingsErrorRef.current) {
      onDefaultSettingsErrorRef.current(
        defaultSettingsError as Error,
      );
    }
  }, [defaultSettingsError]);

  const form = useForm<Settings>({
    defaultValues: settings,
  });

  useEffect(() => {
    if (settings) {
      form.reset(settings);
    }
  }, [settings]);

  const updateSettingsMutation = useMutation({
    mutationFn: (newSettings: Settings) =>
      updateSettings(newSettings),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.settings(),
      });
      form.reset(variables);
      onUpdateSuccessRef.current?.();
    },
    onError: (error) => {
      onUpdateErrorRef.current?.(error as Error);
    },
  });

  const apply = form.handleSubmit((data) => {
    updateSettingsMutation.mutate(data);
  });

  const cancel = () => {
    if (settings) {
      form.reset(settings);
    }
  };

  const resetToDefault = () => {
    if (defaultSettings) {
      form.reset(defaultSettings);
      updateSettingsMutation.mutate(defaultSettings);
    }
  };

  return {
    form,
    isLoading: isLoadingSettings || isLoadingDefaultSettings,
    isUpdating: updateSettingsMutation.isPending,
    apply,
    cancel,
    resetToDefault,
  };
}
