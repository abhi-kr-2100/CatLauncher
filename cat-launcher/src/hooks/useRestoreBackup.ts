import { useMutation, useQueryClient } from "@tanstack/react-query";

import { restoreBackupById } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { GameVariant } from "@/generated-types/GameVariant";

interface UseRestoreBackupOptions {
  onSuccess?: () => void;
  onError?: (error: unknown) => void;
}

export function useRestoreBackup(
  variant: GameVariant,
  { onSuccess, onError }: UseRestoreBackupOptions = {}
) {
  const queryClient = useQueryClient();

  const { mutate: restoreBackup } = useMutation({
    mutationFn: (id: number) => restoreBackupById(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.backups(variant) });
      onSuccess?.();
    },
    onError,
  });

  return { restoreBackup };
}
