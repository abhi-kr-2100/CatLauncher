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
    onErrorRef.current = options.onError;
  }, [options.onSuccess, options.onError]);

  const { mutate, error } = useMutation({
    mutationFn: async (id: bigint) => {
      await restoreManualBackupById(id);
    },
    onSuccess: () => {
      onSuccessRef.current?.();
    },
  });

  useEffect(() => {
    if (error && onErrorRef.current) {
      onErrorRef.current(error);
    }
  }, [error]);

  return { restoreManualBackup: mutate };
}
