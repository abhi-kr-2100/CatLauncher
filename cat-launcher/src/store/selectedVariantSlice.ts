import { createSlice, type PayloadAction } from "@reduxjs/toolkit";

import { GameVariant } from "@/generated-types/GameVariant";

interface SelectedVariantState {
  variant: GameVariant | null;
}

const initialState: SelectedVariantState = {
  variant: null,
};

export const selectedVariantSlice = createSlice({
  name: "selectedVariant",
  initialState,
  reducers: {
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
