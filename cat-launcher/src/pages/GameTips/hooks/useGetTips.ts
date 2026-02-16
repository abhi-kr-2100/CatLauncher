import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getTips } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useGetTips(
  variant: GameVariant,
  onTipsLoadError?: (error: unknown) => void,
) {
  const onTipsLoadErrorRef = useRef(onTipsLoadError);

  useEffect(() => {
    onTipsLoadErrorRef.current = onTipsLoadError;
  }, [onTipsLoadError]);

  const { data, status, error } = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });

  useEffect(() => {
    if (status === "error" && onTipsLoadErrorRef.current) {
      onTipsLoadErrorRef.current(error);
    }
  }, [status, error]);

  return { data: data ?? [], status, error };
}
