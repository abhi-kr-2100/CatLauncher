import { createSlice, PayloadAction } from "@reduxjs/toolkit";

import type { GameVariant } from "@/generated-types/GameVariant";

/**
 * State structure for the {@link gameSessionSlice}.
 */
interface GameSessionState {
  /**
   * The game variant currently being played, or null if no session is active.
   */
  currentlyPlaying: GameVariant | null;
  /**
   * The version of the game variant currently being played, or null if no session is active.
   */
  currentlyPlayingVersion: string | null;
}

/**
 * The initial state for the {@link gameSessionSlice}.
 */
const initialState: GameSessionState = {
  currentlyPlaying: null,
  currentlyPlayingVersion: null,
};

/**
 * A Redux slice that manages the state of the active game session.
 */
export const gameSessionSlice = createSlice({
  name: "gameSession",
  initialState,
  reducers: {
    /**
     * Sets the currently playing game variant and version.
     *
     * @param state - The current state.
     * @param action - The action containing the variant and version.
     */
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
    /**
     * Clears the currently playing game session state.
     *
     * @param state - The current state.
     */
    clearCurrentlyPlaying: (state) => {
      state.currentlyPlaying = null;
      state.currentlyPlayingVersion = null;
    },
  },
});

export const { setCurrentlyPlaying, clearCurrentlyPlaying } =
  gameSessionSlice.actions;

export default gameSessionSlice.reducer;
