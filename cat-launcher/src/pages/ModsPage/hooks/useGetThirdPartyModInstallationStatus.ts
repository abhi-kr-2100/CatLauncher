import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getThirdPartyModInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
  onInstallationStatusError?: (error: unknown) => void,
) {
  const onInstallationStatusErrorRef = useRef(
    onInstallationStatusError,
  );

  useEffect(() => {
    onInstallationStatusErrorRef.current = onInstallationStatusError;
  }, [onInstallationStatusError]);

  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(variant, modId),
    queryFn: () => getThirdPartyModInstallationStatus(modId, variant),
  });

  useEffect(() => {
    if (query.error && onInstallationStatusErrorRef.current) {
      onInstallationStatusErrorRef.current(query.error);
    }
  }, [query.error]);

  return {
    installationStatus: query.data,
  };
}
