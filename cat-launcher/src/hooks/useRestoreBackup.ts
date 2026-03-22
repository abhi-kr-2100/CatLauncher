import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { restoreBackupById } from "@/lib/commands";

interface UseRestoreBackupOptions {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
}

export function useRestoreBackup({
  onSuccess,
  onError,
}: UseRestoreBackupOptions = {}) {
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
    onErrorRef.current = onError;
  }, [onSuccess, onError]);

  const { mutate: restoreBackup } = useMutation({
    mutationFn: (id: bigint) => restoreBackupById(id),
    onSuccess: () => {
      onSuccessRef.current?.();
    },
    onError: (error) => {
      onErrorRef.current?.(error as Error);
    },
  });

  return { restoreBackup };
}
