import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import {
  listenToReleasesUpdate,
  triggerFetchReleasesForVariant,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener } from "@/lib/utils";

export type ReleaseFetchStatus =
  | "idle"
  | "loading"
  | "success"
  | "error";

export function useReleases(
  variant: GameVariant,
  onReleasesLoadError?: (error: unknown) => void,
  onReleasesTriggerError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onReleasesLoadErrorRef = useRef(onReleasesLoadError);
  const onReleasesTriggerErrorRef = useRef(onReleasesTriggerError);
  const [fetchStatus, setFetchStatus] =
    useState<ReleaseFetchStatus>("idle");

  useEffect(() => {
    onReleasesLoadErrorRef.current = onReleasesLoadError;
  }, [onReleasesLoadError]);

  useEffect(() => {
    onReleasesTriggerErrorRef.current = onReleasesTriggerError;
  }, [onReleasesTriggerError]);

  useEffect(() => {
    const releasesUpdateHandler = (
      payload: ReleasesUpdatePayload,
    ) => {
      if (payload.variant === variant) {
        queryClient.setQueryData(
          queryKeys.releases(variant),
          (old: GameRelease[] = []) => {
            const releaseMap = new Map(
              [...old, ...payload.releases].map((r) => [
                r.version,
                r,
              ]),
            );
            return Array.from(releaseMap.values()).sort(
              (a, b) =>
                new Date(b.created_at).getTime() -
                new Date(a.created_at).getTime(),
            );
          },
        );

        if (payload.status === "Success") {
          setFetchStatus("success");
        } else if (payload.status === "Fetching") {
          setFetchStatus("loading");
        }
      }
    };

    const cleanup = setupEventListener(
      listenToReleasesUpdate,
      releasesUpdateHandler,
      "Failed to subscribe to releases updates.",
    );

    return () => {
      setFetchStatus("idle");
      cleanup();
    };
  }, [variant, queryClient]);

  const { data: releases = [], error: releasesError } = useQuery<
    GameRelease[]
  >({
    queryKey: queryKeys.releases(variant),
    queryFn: () => {
      // The actual data will come via events and update the cache.
      return [];
    },
    staleTime: Infinity,
    initialData: [],
  });

  useEffect(() => {
    if (releasesError && onReleasesLoadErrorRef.current) {
      onReleasesLoadErrorRef.current(releasesError);
    }
  }, [releasesError]);

  const {
    mutate: triggerFetchReleases,
    isPending: isReleasesTriggerLoading,
  } = useMutation({
    mutationFn: triggerFetchReleasesForVariant,
    onMutate: () => {
      setFetchStatus("loading");
    },
    onError: (error: unknown) => {
      setFetchStatus("error");
      if (onReleasesTriggerErrorRef.current) {
        onReleasesTriggerErrorRef.current(error);
      }
    },
  });

  useEffect(() => {
    const shouldFetch =
      fetchStatus === "idle" && releases.length === 0;

    if (shouldFetch) {
      triggerFetchReleases(variant);
    }
  }, [variant, triggerFetchReleases, releases.length, fetchStatus]);

  return {
    releases,
    isLoading:
      (fetchStatus === "loading" || isReleasesTriggerLoading) &&
      releases.length === 0,
  };
}
