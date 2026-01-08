import { configureStore } from "@reduxjs/toolkit";

import gameSessionReducer from "./gameSessionSlice";
import installationProgressReducer from "./installationProgressSlice";
import selectedVariantReducer from "./selectedVariantSlice";

export const store = configureStore({
  reducer: {
    gameSession: gameSessionReducer,
    installationProgress: installationProgressReducer,
    selectedVariant: selectedVariantReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
