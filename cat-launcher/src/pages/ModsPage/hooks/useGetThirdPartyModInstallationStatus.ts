import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getThirdPartyModInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
  onStatusError?: (error: Error) => void,
) {
  const onStatusErrorRef = useRef(onStatusError);

  useEffect(() => {
    onStatusErrorRef.current = onStatusError;
  }, [onStatusError]);

  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(variant, modId),
    queryFn: () => getThirdPartyModInstallationStatus(modId, variant),
  });

  useEffect(() => {
    if (query.error && onStatusErrorRef.current) {
      onStatusErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
