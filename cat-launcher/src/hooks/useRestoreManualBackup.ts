import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { restoreManualBackupById } from "@/lib/commands";

export function useRestoreManualBackup(
  options: {
    onSuccess?: () => void;
    onError?: (error: unknown) => void;
  } = {},
) {
  const onSuccessRef = useRef(options.onSuccess);
  const onErrorRef = useRef(options.onError);

  useEffect(() => {
    onSuccessRef.current = options.onSuccess;
  }, [options.onSuccess]);

  useEffect(() => {
    onErrorRef.current = options.onError;
  }, [options.onError]);

  const { mutate } = useMutation({
    mutationFn: async (id: bigint) => {
      await restoreManualBackupById(id);
    },
    onSuccess: () => {
      onSuccessRef.current?.();
    },
    onError: (err) => {
      onErrorRef.current?.(err);
    },
  });

  return { restoreManualBackup: mutate };
}
