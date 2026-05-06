import { useMutation, useQueryClient } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { deleteManualBackupById } from "@/lib/commands";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that provides a mutation for deleting a manual backup.
 * It implements optimistic updates to the manual backups list.
 *
 * @param variant - The game variant the backup belongs to.
 * @param options - Optional callbacks for success and error states.
 * @returns An object containing the `deleteManualBackup` mutation function.
 */
export function useDeleteManualBackup(
  variant: GameVariant,
  options: {
    /**
     * Callback function executed when the deletion is successful.
     */
    onSuccess?: () => void;
    /**
     * Callback function executed when the deletion fails.
     */
    onError?: (error: unknown) => void;
  } = {},
) {
  const queryClient = useQueryClient();

  const { mutate } = useMutation({
    mutationFn: async (id: bigint) => {
      await deleteManualBackupById(id);
    },
    onMutate: async (id) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.manualBackups(variant),
      });

      const previousBackups = queryClient.getQueryData<
        ManualBackupEntry[]
      >(queryKeys.manualBackups(variant));

      queryClient.setQueryData<ManualBackupEntry[]>(
        queryKeys.manualBackups(variant),
        (old) => (old ?? []).filter((backup) => backup.id !== id),
      );

      return { previousBackups };
    },
    onError: (err, _id, context) => {
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

  return { deleteManualBackup: mutate };
}
