import type { GameVariant } from "@/generated-types/GameVariant";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface InstallationProgressState {
  statusByVariant: Record<
    GameVariant,
    Record<string, InstallationProgressStatus | null>
  >;
}

const initialState: InstallationProgressState = {
  statusByVariant: {
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
    },
  },
});

export const { setInstallationProgress, clearInstallationProgress } =
  installationProgressSlice.actions;

export default installationProgressSlice.reducer;
