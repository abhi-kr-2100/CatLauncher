import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { GameVariant } from "@/generated-types/GameVariant";

interface GameSessionState {
  currentlyPlaying: GameVariant | null;
  currentlyPlayingVersion: string | null;
}

const initialState: GameSessionState = {
  currentlyPlaying: null,
  currentlyPlayingVersion: null,
};

export const gameSessionSlice = createSlice({
  name: "gameSession",
  initialState,
  reducers: {
    setCurrentlyPlaying: (
      state,
      action: PayloadAction<{
        variant: GameVariant;
        version: string;
      }>,
    ) => {
      state.currentlyPlaying = action.payload.variant;
      state.currentlyPlayingVersion = action.payload.version;
    },
    clearCurrentlyPlaying: (state) => {
      state.currentlyPlaying = null;
      state.currentlyPlayingVersion = null;
    },
  },
});

export const { setCurrentlyPlaying, clearCurrentlyPlaying } =
  gameSessionSlice.actions;

export default gameSessionSlice.reducer;
