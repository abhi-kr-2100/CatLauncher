import { useMutation } from "@tanstack/react-query";

import { restoreBackupById } from "@/lib/commands";

/**
 * Options for the {@link useRestoreBackup} hook.
 */
interface UseRestoreBackupOptions {
  /**
   * Callback function executed when the restoration is successful.
   */
  onSuccess?: () => void;
  /**
   * Callback function executed when the restoration fails.
   */
  onError?: (error: unknown) => void;
}

/**
 * A custom hook that provides a mutation for restoring an automatic backup by its ID.
 *
 * @param options - Optional callbacks for success and error states.
 * @returns An object containing the `restoreBackup` mutation function.
 */
export function useRestoreBackup({
  onSuccess,
  onError,
}: UseRestoreBackupOptions = {}) {
  const { mutate: restoreBackup } = useMutation({
    mutationFn: (id: bigint) => restoreBackupById(id),
    onSuccess,
    onError,
  });

  return { restoreBackup };
}
