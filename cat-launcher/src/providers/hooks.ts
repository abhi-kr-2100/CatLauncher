import { useCallback, useEffect, useMemo, useState } from "react";
import { useDispatch } from "react-redux";

import {
  listenToAutoupdateStatus,
  listenToGameEvent,
  onFrontendReady,
} from "@/lib/commands";
import { clearCurrentlyPlaying } from "@/store/gameSessionSlice";
import { toastCL } from "@/lib/utils";

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
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    listenToGameEvent((event) => {
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
    })
      .then((unlistenFn) => {
        if (cancelled) {
          unlistenFn();
        }
        unlisten = unlistenFn;
      })
      .catch((error) => {
        toastCL("error", "Error listening to game events", error);
      });

    return () => {
      cancelled = true;
      unlisten?.();
    };
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
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    listenToAutoupdateStatus((status) => {
      switch (status.type) {
        case "Failure":
          setAutoUpdateStatus(AutoUpdateStatus.FAILURE);
          break;
      }
    })
      .then((unlistenFn) => {
        if (cancelled) {
          unlistenFn();
        }

        unlisten = unlistenFn;
      })
      .catch((error) => {
        toastCL("error", "Error listening to autoupdate status", error);
      });

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return {
    autoUpdateStatus,
    resetAutoUpdateStatus,
  };
}
