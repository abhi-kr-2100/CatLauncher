import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { SerializableDownloadProgress } from "@/lib/types";
import type { GameVariant } from "@/generated-types/GameVariant";

/**
 * Represents the type of component being installed.
 */
type InstallationType = "release" | "mod" | "soundpack" | "tileset";

/**
 * Possible statuses for an ongoing installation.
 */
export type InstallationProgressStatus =
  | "Downloading"
  | "Installing"
  | "Success"
  | "Error";

/**
 * State structure for the {@link installationProgressSlice}.
 */
interface InstallationProgressState {
  /**
   * A nested record mapping installation type, game variant, and item ID to its installation status.
   */
  installationStatusByVariant: Record<
    InstallationType,
    Record<
      GameVariant,
      Record<string, InstallationProgressStatus | null>
    >
  >;
  /**
   * A nested record mapping installation type, game variant, and item ID to its download progress.
   */
  downloadProgressByVariant: Record<
    InstallationType,
    Record<
      GameVariant,
      Record<string, SerializableDownloadProgress | null>
    >
  >;
}

/**
 * The initial state for the {@link installationProgressSlice}.
 */
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

/**
 * A Redux slice that manages the progress and status of ongoing installations.
 */
export const installationProgressSlice = createSlice({
  name: "installationProgress",
  initialState,
  reducers: {
    /**
     * Updates the download progress and derives the installation status for a specific item.
     *
     * @param state - The current state.
     * @param action - The action containing progress details.
     */
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

    /**
     * Clears the installation progress and status for a specific item.
     *
     * @param state - The current state.
     * @param action - The action containing the item details to clear.
     */
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
