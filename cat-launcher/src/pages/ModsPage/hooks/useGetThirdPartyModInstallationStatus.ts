import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getThirdPartyModInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(variant, modId),
    queryFn: () => getThirdPartyModInstallationStatus(modId, variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    installationStatus: query.data,
  };
}
