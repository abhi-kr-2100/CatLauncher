import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";

export enum FetchStatus {
  Idle = "Idle",
  Fetching = "Fetching",
  Error = "Error",
}

interface ReleasesState {
  releasesByVariant: Record<GameVariant, GameRelease[]>;
  fetchStatusByVariant: Record<GameVariant, FetchStatus>;
}

const initialState: ReleasesState = {
  releasesByVariant: {
    BrightNights: [],
    DarkDaysAhead: [],
    TheLastGeneration: [],
  },
  fetchStatusByVariant: {
    BrightNights: FetchStatus.Idle,
    DarkDaysAhead: FetchStatus.Idle,
    TheLastGeneration: FetchStatus.Idle,
  },
};

export const releasesSlice = createSlice({
  name: "releases",
  initialState,
  reducers: {
    onFetchingReleasesFailed: (
      state,
      action: PayloadAction<{ variant: GameVariant }>,
    ) => {
      state.fetchStatusByVariant[action.payload.variant] =
        FetchStatus.Error;
    },
    startFetchingReleases: (
      state,
      action: PayloadAction<{ variant: GameVariant }>,
    ) => {
      state.fetchStatusByVariant[action.payload.variant] =
        FetchStatus.Fetching;
    },
    updateReleasesForVariant: (
      state,
      action: PayloadAction<ReleasesUpdatePayload>,
    ) => {
      const { variant, releases, status } = action.payload;
      const oldReleases = state.releasesByVariant[variant];
      const newReleases = releases;

      const releaseMap = new Map(
        [...oldReleases, ...newReleases].map((r) => [r.version, r]),
      );
      state.releasesByVariant[variant] = Array.from(
        releaseMap.values(),
      ).sort(
        (a, b) =>
          new Date(b.created_at).getTime() -
          new Date(a.created_at).getTime(),
      );

      if (status === "Success") {
        state.fetchStatusByVariant[variant] = FetchStatus.Idle;
      } else if (status === "Error") {
        state.fetchStatusByVariant[variant] = FetchStatus.Error;
      }
    },
  },
});

export const {
  onFetchingReleasesFailed,
  startFetchingReleases,
  updateReleasesForVariant,
} = releasesSlice.actions;

export default releasesSlice.reducer;
