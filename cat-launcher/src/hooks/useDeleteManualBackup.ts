import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { deleteManualBackupById } from "@/lib/commands";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import { queryKeys } from "@/lib/queryKeys";

export function useDeleteManualBackup(
  variant: GameVariant,
  options: {
    onSuccess?: () => void;
    onError?: (error: Error) => void;
  } = {},
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(options.onSuccess);
  const onErrorRef = useRef(options.onError);

  useEffect(() => {
    onSuccessRef.current = options.onSuccess;
    onErrorRef.current = options.onError;
  }, [options.onSuccess, options.onError]);

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
      onErrorRef.current?.(err as Error);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.manualBackups(variant),
      });
    },
    onSuccess: () => {
      onSuccessRef.current?.();
    },
  });

  return { deleteManualBackup: mutate };
}
