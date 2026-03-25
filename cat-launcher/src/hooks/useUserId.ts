import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getUserId } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useUserId(
  onUserIdFetchError?: (error: Error) => void,
) {
  const onUserIdFetchErrorRef = useRef(onUserIdFetchError);

  useEffect(() => {
    onUserIdFetchErrorRef.current = onUserIdFetchError;
  }, [onUserIdFetchError]);

  const query = useQuery({
    queryKey: queryKeys.userId(),
    queryFn: getUserId,
  });

  useEffect(() => {
    if (query.error && onUserIdFetchErrorRef.current) {
      onUserIdFetchErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    userId: query.data,
  };
}
