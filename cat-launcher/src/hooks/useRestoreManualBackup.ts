import { useMutation } from "@tanstack/react-query";

import { restoreManualBackupById } from "@/lib/commands";

export function useRestoreManualBackup(
  options: {
    onSuccess?: () => void;
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
