import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { deleteBackupById } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { GameVariant } from "@/generated-types/GameVariant";
import { BackupEntry } from "@/generated-types/BackupEntry";

interface UseDeleteBackupOptions {
  onSuccess?: () => void;
  onError?: (error: unknown) => void;
}

export function useDeleteBackup(
  variant: GameVariant,
  { onSuccess, onError }: UseDeleteBackupOptions = {},
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
  }, [onSuccess]);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

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
    onSuccess: () => {
      onSuccessRef.current?.();
    },
    onError: (error, _variables, context) => {
      if (context?.previousBackups) {
        queryClient.setQueryData(
          queryKeys.backups(variant),
          context.previousBackups,
        );
      }
      onErrorRef.current?.(error);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.backups(variant),
      });
    },
  });

  return { deleteBackup };
}
