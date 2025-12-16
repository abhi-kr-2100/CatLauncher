import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { SerializableDownloadProgress } from "@/lib/types";
import type { GameVariant } from "@/generated-types/GameVariant";

type InstallationType = "release" | "mod" | "soundpack" | "tileset";

export type InstallationProgressStatus =
  | "Downloading"
  | "Installing"
  | "Success"
  | "Error";

interface InstallationProgressState {
  installationStatusByVariant: Record<
    InstallationType,
    Record<
      GameVariant,
      Record<string, InstallationProgressStatus | null>
    >
  >;
  downloadProgressByVariant: Record<
    InstallationType,
    Record<
      GameVariant,
      Record<string, SerializableDownloadProgress | null>
    >
  >;
}

const initialState: InstallationProgressState = {
  installationStatusByVariant: {
    release: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    mod: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    soundpack: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    tileset: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
  },
  downloadProgressByVariant: {
    release: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    mod: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    soundpack: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
    tileset: {
      BrightNights: {},
      DarkDaysAhead: {},
      TheLastGeneration: {},
    },
  },
};

export const installationProgressSlice = createSlice({
  name: "installationProgress",
  initialState,
  reducers: {
    setDownloadProgress: (
      state,
      action: PayloadAction<{
        type: InstallationType;
        variant: GameVariant;
        id: string;
        progress: SerializableDownloadProgress;
      }>,
    ) => {
      const { type, variant, id, progress } = action.payload;
      state.downloadProgressByVariant[type][variant][id] = progress;

      const { bytes_downloaded, total_bytes } = progress;

      if (total_bytes === 0) {
        state.installationStatusByVariant[type][variant][id] =
          "Downloading";
      } else if (bytes_downloaded === total_bytes) {
        state.installationStatusByVariant[type][variant][id] =
          "Installing";
      } else {
        state.installationStatusByVariant[type][variant][id] =
          "Downloading";
      }
    },

    clearInstallationProgress: (
      state,
      action: PayloadAction<{
        variant: GameVariant;
        id: string;
        type: InstallationType;
      }>,
    ) => {
      const { variant, id, type } = action.payload;

      state.installationStatusByVariant[type][variant][id] = null;
      state.downloadProgressByVariant[type][variant][id] = null;
    },
  },
});

export const { setDownloadProgress, clearInstallationProgress } =
  installationProgressSlice.actions;

export default installationProgressSlice.reducer;
