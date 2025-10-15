import { useCallback, useEffect, useMemo, useState } from "react";
import { useDispatch } from "react-redux";

import { GameEvent } from "@/generated-types/GameEvent";
import { UpdateStatus } from "@/generated-types/UpdateStatus";
import {
  listenToAutoupdateStatus,
  listenToGameEvent,
  onFrontendReady,
} from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
import { clearCurrentlyPlaying } from "@/store/gameSessionSlice";

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

  const logsText = useMemo(() => logs.join("\n"), [logs]);

  const resetGameSessionMonitor = useCallback(() => {
    setLogs([]);
    setGameStatus(GameStatus.IDLE);
  }, []);

  const dispatch = useDispatch();

  useEffect(() => {
    const gameEventHandler = (event: GameEvent) => {
      switch (event.type) {
        case "Log":
          setLogs((prev) => [...prev, event.payload]);
          break;
        case "Exit":
          dispatch(clearCurrentlyPlaying());
          // code is null if the process was terminated by a signal
          if (event.payload.code !== 0) {
            setGameStatus(GameStatus.CRASHED);
          } else {
            // Game exited successfully, clear logs
            setLogs([]);
          }
          break;
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

  return { gameStatus, logsText, resetGameSessionMonitor };
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
