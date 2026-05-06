import { useMutation, useQueryClient } from "@tanstack/react-query";

import { deleteBackupById } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { GameVariant } from "@/generated-types/GameVariant";
import { BackupEntry } from "@/generated-types/BackupEntry";

/**
 * Options for the {@link useDeleteBackup} hook.
 */
interface UseDeleteBackupOptions {
  /**
   * Callback function executed when the deletion is successful.
   */
  onSuccess?: () => void;
  /**
   * Callback function executed when the deletion fails.
   */
  onError?: (error: unknown) => void;
}

/**
 * A custom hook that provides a mutation for deleting an automatic backup.
 * It implements optimistic updates to the backups list.
 *
 * @param variant - The game variant the backup belongs to.
 * @param options - Optional callbacks for success and error states.
 * @returns An object containing the `deleteBackup` mutation function.
 */
export function useDeleteBackup(
  variant: GameVariant,
  { onSuccess, onError }: UseDeleteBackupOptions = {},
) {
  const queryClient = useQueryClient();

  const { mutate: deleteBackup } = useMutation({
    mutationFn: (id: bigint) => deleteBackupById(id),
    onMutate: async (id: bigint) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.backups(variant),
      });

      const previousBackups = queryClient.getQueryData<BackupEntry[]>(
        queryKeys.backups(variant),
      );

      queryClient.setQueryData<BackupEntry[]>(
        queryKeys.backups(variant),
        (old) => old?.filter((backup) => backup.id !== id) ?? [],
      );

      return { previousBackups };
    },
    onSuccess,
    onError: (error, _variables, context) => {
      if (context?.previousBackups) {
        queryClient.setQueryData(
          queryKeys.backups(variant),
          context.previousBackups,
        );
      }
      onError?.(error);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.backups(variant),
      });
    },
  });

  return { deleteBackup };
}
