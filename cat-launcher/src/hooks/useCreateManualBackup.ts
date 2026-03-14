import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { createManualBackupForVariant } from "@/lib/commands";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import { queryKeys } from "@/lib/queryKeys";

export function useCreateManualBackup(
  variant: GameVariant,
  options: {
    onSuccess?: () => void;
    onError?: (error: unknown) => void;
  } = {},
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(options.onSuccess);
  const onErrorRef = useRef(options.onError);

  useEffect(() => {
    onSuccessRef.current = options.onSuccess;
    onErrorRef.current = options.onError;
  }, [options.onSuccess, options.onError]);

  const { mutate, isPending, error } = useMutation({
    mutationFn: async (values: { name: string; notes?: string }) => {
      await createManualBackupForVariant(
        variant,
        values.name,
        values.notes,
      );
    },
    onMutate: async (newBackup) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.manualBackups(variant),
      });

      const previousBackups = queryClient.getQueryData<
        ManualBackupEntry[]
      >(queryKeys.manualBackups(variant));

      queryClient.setQueryData<ManualBackupEntry[]>(
        queryKeys.manualBackups(variant),
        (old) => [
          ...(old ?? []),
          {
            id: BigInt(-1), // Temporary ID
            name: newBackup.name,
            game_variant: variant,
            timestamp: BigInt(Math.floor(Date.now() / 1000)),
            notes: newBackup.notes ?? null,
          },
        ],
      );

      return { previousBackups };
    },
    onError: (_err, _newBackup, context) => {
      queryClient.setQueryData(
        queryKeys.manualBackups(variant),
        context?.previousBackups,
      );
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.manualBackups(variant),
      });
    },
    onSuccess: () => {
      onSuccessRef.current?.();
    },
  });

  useEffect(() => {
    if (error && onErrorRef.current) {
      onErrorRef.current(error);
    }
  }, [error]);

  return {
    createManualBackup: mutate,
    isCreatingManualBackup: isPending,
  };
}
