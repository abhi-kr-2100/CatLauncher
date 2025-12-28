import { useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { logPlayTime } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";

const DURATION_TO_LOG_SECONDS = 60;
const INTERVAL_MS = DURATION_TO_LOG_SECONDS * 1000;

export function usePlayTimeMonitor(
  currentlyPlaying: GameVariant | null,
  currentlyPlayingVersion: string | null,
) {
  const queryClient = useQueryClient();
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(
    null,
  );
  const inFlightRef = useRef(false);

  useEffect(() => {
    if (!currentlyPlaying || !currentlyPlayingVersion) {
      return;
    }

    const scheduleNextLog = async () => {
      // Skip if a log is already in flight to prevent overlapping calls
      if (inFlightRef.current) {
        return;
      }

      inFlightRef.current = true;
      try {
        await logPlayTime(
          currentlyPlaying,
          currentlyPlayingVersion,
          DURATION_TO_LOG_SECONDS,
        );

        // Invalidate queries to update UI in real-time if PlayPage is open
        queryClient.invalidateQueries({
          queryKey: queryKeys.playTimeForVariant(currentlyPlaying),
        });
        queryClient.invalidateQueries({
          queryKey: queryKeys.playTimeForVersion(
            currentlyPlaying,
            currentlyPlayingVersion,
          ),
        });
      } catch (error) {
        toastCL("error", "Failed to log play time.", error);
      } finally {
        inFlightRef.current = false;
        timeoutRef.current = setTimeout(scheduleNextLog, INTERVAL_MS);
      }
    };

    // Start the first log immediately
    scheduleNextLog();

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [currentlyPlaying, currentlyPlayingVersion, queryClient]);
}
