import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { getTips } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetTips(
  variant: GameVariant,
  onTipsLoadError?: (error: unknown) => void,
) {
  const onTipsLoadErrorRef = useRef(onTipsLoadError);

  useEffect(() => {
    onTipsLoadErrorRef.current = onTipsLoadError;
  }, [onTipsLoadError]);

  const {
    data: tips = [],
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: () => getTips(variant),
  });

  useEffect(() => {
    if (error && onTipsLoadErrorRef.current) {
      onTipsLoadErrorRef.current(error);
    }
  }, [error]);

  return { tips, isLoading };
}
