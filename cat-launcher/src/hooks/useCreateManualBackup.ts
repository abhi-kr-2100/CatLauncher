import { useMutation, useQueryClient } from "@tanstack/react-query";

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

  const { mutate } = useMutation({
    mutationFn: async (values: { name: string; notes?: string }) => {
      await createManualBackupForVariant(variant, values.name, values.notes);
    },
    onMutate: async (newBackup) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.manualBackups(variant),
      });

      const previousBackups = queryClient.getQueryData<ManualBackupEntry[]>(
        queryKeys.manualBackups(variant),
      );

      queryClient.setQueryData<ManualBackupEntry[]>(
        queryKeys.manualBackups(variant),
        (old) => [
          ...(old ?? []),
          {
            ...newBackup,
            id: -1, // Temporary ID
            game_variant: variant,
            timestamp: Date.now() / 1000,
          },
        ],
      );

      return { previousBackups };
    },
    onError: (err, newBackup, context) => {
      queryClient.setQueryData(
        queryKeys.manualBackups(variant),
        context?.previousBackups,
      );
      options.onError?.(err);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.manualBackups(variant),
      });
    },
    onSuccess: options.onSuccess,
  });

  return { createManualBackup: mutate };
}
