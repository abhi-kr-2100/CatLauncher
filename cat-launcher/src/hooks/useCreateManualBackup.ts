import { useMutation, useQueryClient } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { createManualBackupForVariant } from "@/lib/commands";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that provides a mutation for creating a manual backup for a game variant.
 * It implements optimistic updates to the manual backups list.
 *
 * @param variant - The game variant to create a backup for.
 * @param options - Optional callbacks for success and error states.
 * @returns An object containing the `createManualBackup` mutation function and its pending state.
 */
export function useCreateManualBackup(
  variant: GameVariant,
  options: {
    /**
     * Callback function executed when the creation is successful.
     */
    onSuccess?: () => void;
    /**
     * Callback function executed when the creation fails.
     */
    onError?: (error: unknown) => void;
  } = {},
) {
  const queryClient = useQueryClient();

  const { mutate, isPending } = useMutation({
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
    onError: (err, _newBackup, context) => {
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

  return {
    createManualBackup: mutate,
    isCreatingManualBackup: isPending,
  };
}
