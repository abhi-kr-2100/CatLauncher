import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getUserId } from "@/lib/commands";

export function useUserId(onUserIdError?: (error: Error) => void) {
  const onUserIdErrorRef = useRef(onUserIdError);

  useEffect(() => {
    onUserIdErrorRef.current = onUserIdError;
  }, [onUserIdError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.userId(),
    queryFn: getUserId,
  });

  useEffect(() => {
    if (error && onUserIdErrorRef.current) {
      onUserIdErrorRef.current(error as Error);
    }
  }, [error]);

  return { userId: data, isLoading };
}
