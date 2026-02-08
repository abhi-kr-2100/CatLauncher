import { useEffect, useRef } from "react";
import { useQuery } from "@tanstack/react-query";

import { getGuideEntity } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import type { GameVariant } from "@/generated-types/GameVariant";

export default function useGuideEntity(
  variant: GameVariant,
  id: string | null,
  onEntityError?: (error: Error) => void,
) {
  const onEntityErrorRef = useRef(onEntityError);

  useEffect(() => {
    onEntityErrorRef.current = onEntityError;
  }, [onEntityError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.guide.entity(variant, id ?? ""),
    queryFn: () => getGuideEntity(id!, variant),
    enabled: !!id,
  });

  useEffect(() => {
    if (error && onEntityErrorRef.current) {
      onEntityErrorRef.current(
        error instanceof Error ? error : new Error(String(error)),
      );
    }
  }, [error]);

  return { data, isLoading, error };
}
