import { useCallback, useEffect, useMemo, useState } from "react";

import { GameEvent } from "@/generated-types/GameEvent";
import { UpdateStatus } from "@/generated-types/UpdateStatus";
import {
  listenToAutoupdateStatus,
  listenToGameEvent,
  onFrontendReady,
} from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
import { clearCurrentlyPlaying } from "@/store/gameSessionSlice";
import { useAppDispatch, useAppSelector } from "@/store/hooks";

export function useFrontendReady() {
  useEffect(() => {
    onFrontendReady();
  }, []);
}

export enum GameStatus {
  IDLE = "IDLE",
  CRASHED = "CRASHED",
  ERROR = "ERROR",
  TERMINATED = "TERMINATED",
}

export function useGameSessionEvents() {
  const [gameStatus, setGameStatus] = useState<GameStatus>(GameStatus.IDLE);
  const [logs, setLogs] = useState<string[]>([]);
  const [exitCode, setExitCode] = useState<number | null | undefined>(
    undefined,
  );

  const logsText = useMemo(() => logs.join("\n"), [logs]);

  const resetGameSessionMonitor = useCallback(() => {
    setLogs([]);
    setGameStatus(GameStatus.IDLE);
    setExitCode(undefined);
  }, []);

  const dispatch = useAppDispatch();

  const currentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying,
  );

  useEffect(() => {
    const gameEventHandler = (event: GameEvent) => {
      switch (event.type) {
        case "Log":
          setLogs((prev) => [...prev, event.payload]);
          break;
        case "Exit": {
          dispatch(clearCurrentlyPlaying());
          const code = event.payload.code;
          setExitCode(code);

          if (code === null) {
            // Game was terminated by signal (null)
            setGameStatus(GameStatus.TERMINATED);
          } else if (code === 0 || currentlyPlaying === "BrightNights") {
            // BrightNights returns non-zero exit code almost always, even if it exited
            // successfully. To not overwhelm the user, we don't show crash logs for it.
            resetGameSessionMonitor();
          } else {
            setGameStatus(GameStatus.CRASHED);
          }
          break;
        }
        case "Error":
          dispatch(clearCurrentlyPlaying());
          setLogs((prev) => [...prev, `ERROR: ${event.payload.message}`]);
          setGameStatus(GameStatus.ERROR);
          break;
      }
    };

    const cleanup = setupEventListener(
      listenToGameEvent,
      gameEventHandler,
      "Error listening to game events.",
    );

    return cleanup;
  }, [dispatch, currentlyPlaying, resetGameSessionMonitor]);

  return { gameStatus, logsText, exitCode, resetGameSessionMonitor };
}

export enum AutoUpdateStatus {
  IDLE = "IDLE",
  FAILURE = "FAILURE",
}

export function useAutoUpdateEvents() {
  const [autoUpdateStatus, setAutoUpdateStatus] = useState<AutoUpdateStatus>(
    AutoUpdateStatus.IDLE,
  );
  const resetAutoUpdateStatus = useCallback(() => {
    setAutoUpdateStatus(AutoUpdateStatus.IDLE);
  }, []);

  useEffect(() => {
    const autoUpdateHandler = (status: UpdateStatus) => {
      switch (status.type) {
        case "Failure":
          setAutoUpdateStatus(AutoUpdateStatus.FAILURE);
          break;
      }
    };

    const cleanup = setupEventListener(
      listenToAutoupdateStatus,
      autoUpdateHandler,
      "Error listening to autoupdate status.",
    );

    return cleanup;
  }, []);

  return {
    autoUpdateStatus,
    resetAutoUpdateStatus,
  };
}
