import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { GameVariant } from "@/generated-types/GameVariant";

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

export default gameSessionSlice.reducer;
