import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { restoreBackupById } from "@/lib/commands";

interface UseRestoreBackupOptions {
  onSuccess?: () => void;
  onError?: (error: unknown) => void;
}

export function useRestoreBackup({
  onSuccess,
  onError,
}: UseRestoreBackupOptions = {}) {
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
  }, [onSuccess]);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const { mutate: restoreBackup } = useMutation({
    mutationFn: (id: bigint) => restoreBackupById(id),
    onSuccess: () => {
      onSuccessRef.current?.();
    },
    onError: (error) => {
      onErrorRef.current?.(error);
    },
  });

  return { restoreBackup };
}
