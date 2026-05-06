import { useMutation } from "@tanstack/react-query";

import { restoreManualBackupById } from "@/lib/commands";

/**
 * A custom hook that provides a mutation for restoring a manual backup by its ID.
 *
 * @param options - Optional callbacks for success and error states.
 * @returns An object containing the `restoreManualBackup` mutation function.
 */
export function useRestoreManualBackup(
  options: {
    /**
     * Callback function executed when the restoration is successful.
     */
    onSuccess?: () => void;
    /**
     * Callback function executed when the restoration fails.
     */
    onError?: (error: unknown) => void;
  } = {},
) {
  const { mutate } = useMutation({
    mutationFn: async (id: bigint) => {
      await restoreManualBackupById(id);
    },
    onSuccess: options.onSuccess,
    onError: options.onError,
  });

  return { restoreManualBackup: mutate };
}
