import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import { GameVariant } from "@/generated-types/GameVariant";

/**
 * State structure for the {@link selectedVariantSlice}.
 */
interface SelectedVariantState {
  /**
   * The currently selected game variant, or null if none is selected.
   */
  variant: GameVariant | null;
}

/**
 * The initial state for the {@link selectedVariantSlice}.
 */
const initialState: SelectedVariantState = {
  variant: null,
};

/**
 * A Redux slice that manages the currently selected game variant.
 */
export const selectedVariantSlice = createSlice({
  name: "selectedVariant",
  initialState,
  reducers: {
    /**
     * Sets the currently selected game variant.
     *
     * @param state - The current state.
     * @param action - The action containing the new variant.
     */
    setSelectedVariant: (
      state,
      action: PayloadAction<GameVariant | null>,
    ) => {
      state.variant = action.payload;
    },
  },
});

export const { setSelectedVariant } = selectedVariantSlice.actions;
export default selectedVariantSlice.reducer;
