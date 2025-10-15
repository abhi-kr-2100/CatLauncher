import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { RootState } from "./store";

interface GameSessionState {
  currentlyPlaying: GameVariant | null;
}

const initialState: GameSessionState = {
  currentlyPlaying: null,
};

export const gameSessionSlice = createSlice({
  name: "gameSession",
  initialState,
  reducers: {
    setCurrentlyPlaying: (
      state,
      action: PayloadAction<{ variant: GameVariant }>,
    ) => {
      state.currentlyPlaying = action.payload.variant;
    },
    clearCurrentlyPlaying: (state) => {
      state.currentlyPlaying = null;
    },
  },
});

export const { setCurrentlyPlaying, clearCurrentlyPlaying } =
  gameSessionSlice.actions;

export const selectCurrentlyPlaying = (state: RootState) =>
  state.gameSession.currentlyPlaying;

export default gameSessionSlice.reducer;
