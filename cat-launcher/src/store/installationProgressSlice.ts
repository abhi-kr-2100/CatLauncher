import type { DownloadProgress } from "@/generated-types/DownloadProgress";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface InstallationProgressState {
  statusByVariant: Record<
    GameVariant,
    Record<string, InstallationProgressStatus | null>
  >;
  progressByVariant: Record<
    GameVariant,
    Record<string, DownloadProgress | null>
  >;
}

const initialState: InstallationProgressState = {
  statusByVariant: {
    BrightNights: {},
    DarkDaysAhead: {},
    TheLastGeneration: {},
  },
  progressByVariant: {
    BrightNights: {},
    DarkDaysAhead: {},
    TheLastGeneration: {},
  },
};

export const installationProgressSlice = createSlice({
  name: "installationProgress",
  initialState,
  reducers: {
    setInstallationProgress: (
      state,
      action: PayloadAction<{
        variant: GameVariant;
        releaseId: string;
        status: InstallationProgressStatus;
      }>,
    ) => {
      const { variant, releaseId, status } = action.payload;
      if (!state.statusByVariant[variant]) {
        state.statusByVariant[variant] = {};
      }
      state.statusByVariant[variant][releaseId] = status;
    },

    setDownloadProgress: (
      state,
      action: PayloadAction<{
        variant: GameVariant;
        releaseId: string;
        progress: DownloadProgress;
      }>,
    ) => {
      const { variant, releaseId, progress } = action.payload;
      if (!state.progressByVariant[variant]) {
        state.progressByVariant[variant] = {};
      }
      state.progressByVariant[variant][releaseId] = progress;
    },

    clearInstallationProgress: (
      state,
      action: PayloadAction<{
        variant: GameVariant;
        releaseId: string | undefined;
      }>,
    ) => {
      const { variant, releaseId } = action.payload;
      if (releaseId && state.statusByVariant[variant]) {
        state.statusByVariant[variant][releaseId] = null;
      }
      if (releaseId && state.progressByVariant[variant]) {
        state.progressByVariant[variant][releaseId] = null;
      }
    },
  },
});

export const {
  setInstallationProgress,
  clearInstallationProgress,
  setDownloadProgress,
} = installationProgressSlice.actions;

export default installationProgressSlice.reducer;
