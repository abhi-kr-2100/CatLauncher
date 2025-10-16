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
import { useAppDispatch } from "@/store/hooks";

export function useFrontendReady() {
  useEffect(() => {
    onFrontendReady();
  }, []);
}

export enum GameStatus {
  IDLE = "IDLE",
  CRASHED = "CRASHED",
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

          if (code === 0) {
            // Game exited successfully
            resetGameSessionMonitor();
          } else {
            // Game crashed (non-zero exit code) or was terminated by signal (null)
            setGameStatus(GameStatus.CRASHED);
          }
          break;
        }
        case "Error":
          dispatch(clearCurrentlyPlaying());
          setLogs((prev) => [...prev, `ERROR: ${event.payload.message}`]);
          setGameStatus(GameStatus.CRASHED);
          break;
      }
    };

    const cleanup = setupEventListener(
      listenToGameEvent,
      gameEventHandler,
      "Error listening to game events.",
    );

    return cleanup;
  }, [dispatch]);

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
