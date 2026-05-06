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

/**
 * Hook that notifies the backend that the frontend is ready.
 * This should be called once when the application starts.
 */
export function useFrontendReady() {
  useEffect(() => {
    onFrontendReady();
  }, []);
}

/**
 * Represents the status of a game session.
 */
export enum GameStatus {
  /** The game is not running or has exited normally. */
  IDLE = "IDLE",
  /** The game crashed with a non-zero exit code. */
  CRASHED = "CRASHED",
  /** An error occurred while trying to launch or monitor the game. */
  ERROR = "ERROR",
  /** The game was terminated by an external signal. */
  TERMINATED = "TERMINATED",
}

/**
 * Hook that listens for game session events (logs, exits, errors) and manages the game status and logs.
 *
 * @returns An object containing the current game status, logs as text, exit code, and a function to reset the monitor.
 */
export function useGameSessionEvents() {
  const [gameStatus, setGameStatus] = useState<GameStatus>(
    GameStatus.IDLE,
  );
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
          } else if (
            code === 0 ||
            currentlyPlaying === "BrightNights"
          ) {
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
          setLogs((prev) => [
            ...prev,
            `ERROR: ${event.payload.message}`,
          ]);
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

/**
 * Represents the status of the application's auto-update process.
 */
export enum AutoUpdateStatus {
  /** No update is in progress or an update succeeded. */
  IDLE = "IDLE",
  /** The auto-update process failed. */
  FAILURE = "FAILURE",
}

/**
 * Hook that listens for auto-update status events and manages the auto-update status.
 *
 * @returns An object containing the current auto-update status and a function to reset it.
 */
export function useAutoUpdateEvents() {
  const [autoUpdateStatus, setAutoUpdateStatus] =
    useState<AutoUpdateStatus>(AutoUpdateStatus.IDLE);
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
