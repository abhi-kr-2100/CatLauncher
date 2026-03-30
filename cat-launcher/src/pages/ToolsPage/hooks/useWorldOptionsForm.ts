import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef } from "react";
import { useForm } from "react-hook-form";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { WorldOption } from "@/generated-types/WorldOption";
import { getWorldOptions, updateWorldOptions } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export interface UseWorldOptionsFormProps {
  variant: GameVariant;
  worldName: string;
  onWorldOptionsError?: (error: Error) => void;
  onUpdateError?: (error: Error) => void;
  onUpdateSuccess?: () => void;
}

export interface WorldOptionsFormData {
  options: WorldOption[];
}

export function useWorldOptionsForm({
  variant,
  worldName,
  onWorldOptionsError,
  onUpdateError,
  onUpdateSuccess,
}: UseWorldOptionsFormProps) {
  const queryClient = useQueryClient();

  const onWorldOptionsErrorRef = useRef(onWorldOptionsError);
  const onUpdateErrorRef = useRef(onUpdateError);
  const onUpdateSuccessRef = useRef(onUpdateSuccess);

  useEffect(() => {
    onWorldOptionsErrorRef.current = onWorldOptionsError;
    onUpdateErrorRef.current = onUpdateError;
    onUpdateSuccessRef.current = onUpdateSuccess;
  }, [onWorldOptionsError, onUpdateError, onUpdateSuccess]);

  const {
    data: options,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.worldOptions(variant, worldName),
    queryFn: () => getWorldOptions(variant, worldName),
    enabled: !!worldName,
  });

  useEffect(() => {
    if (error && onWorldOptionsErrorRef.current) {
      onWorldOptionsErrorRef.current(error as Error);
    }
  }, [error]);

  const form = useForm<WorldOptionsFormData>({
    mode: "onChange",
    defaultValues: {
      options: options ?? [],
    },
  });

  useEffect(() => {
    if (options) {
      form.reset({ options });
    }
  }, [options, form]);

  const updateMutation = useMutation({
    mutationFn: (data: WorldOptionsFormData) =>
      updateWorldOptions(variant, worldName, data.options),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.worldOptions(variant, worldName),
      });
      form.reset(variables);
      onUpdateSuccessRef.current?.();
    },
    onError: (error) => {
      onUpdateErrorRef.current?.(error as Error);
    },
  });

  const apply = form.handleSubmit((data) => {
    updateMutation.mutate(data);
  });

  const cancel = () => {
    if (options) {
      form.reset({ options });
    }
  };

  return {
    form,
    isLoading,
    isUpdating: updateMutation.isPending,
    apply,
    cancel,
  };
}
