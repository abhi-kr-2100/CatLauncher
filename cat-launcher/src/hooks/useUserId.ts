import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getUserId } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useUserId(onUserIdError?: (error: Error) => void) {
  const onUserIdErrorRef = useRef(onUserIdError);

  useEffect(() => {
    onUserIdErrorRef.current = onUserIdError;
  }, [onUserIdError]);

  const {
    data: userId,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.userId(),
    queryFn: getUserId,
  });

  useEffect(() => {
    if (error && onUserIdErrorRef.current) {
      onUserIdErrorRef.current(error as Error);
    }
  }, [error]);

  return { userId, isLoading, error };
}
