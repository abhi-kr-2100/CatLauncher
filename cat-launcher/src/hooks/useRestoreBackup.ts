import { useMutation } from "@tanstack/react-query";

import { restoreBackupById } from "@/lib/commands";

interface UseRestoreBackupOptions {
  onSuccess?: () => void;
  onError?: (error: unknown) => void;
}

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
