import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { RootState } from "./store";

interface ReleasesState {
  releasesByVariant: Record<GameVariant, GameRelease[]>;
}

const initialState: ReleasesState = {
  releasesByVariant: {
    BrightNights: [],
    DarkDaysAhead: [],
    TheLastGeneration: [],
  },
};

export const releasesSlice = createSlice({
  name: "releases",
  initialState,
  reducers: {
    updateReleasesForVariant: (
      state,
      action: PayloadAction<{ variant: GameVariant; releases: GameRelease[] }>
    ) => {
      const { variant, releases } = action.payload;
      const oldReleases = state.releasesByVariant[variant];
      const newReleases = releases;

      const releaseMap = new Map(
        [...oldReleases, ...newReleases].map((r) => [r.version, r])
      );
      state.releasesByVariant[variant] = Array.from(releaseMap.values()).sort(
        (a, b) =>
          new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      );
    },
  },
});

export const { updateReleasesForVariant } = releasesSlice.actions;

export const selectReleasesForVariant = (
  state: RootState,
  variant: GameVariant
) => state.releases.releasesByVariant[variant];

export default releasesSlice.reducer;
