import { configureStore } from "@reduxjs/toolkit";

import gameSessionReducer from "./gameSessionSlice";
import installationProgressReducer from "./installationProgressSlice";
import selectedVariantReducer from "./selectedVariantSlice";

/**
 * The root Redux store for the application.
 */
export const store = configureStore({
  reducer: {
    gameSession: gameSessionReducer,
    installationProgress: installationProgressReducer,
    selectedVariant: selectedVariantReducer,
  },
});

/**
 * The root state type of the Redux store.
 */
export type RootState = ReturnType<typeof store.getState>;
/**
 * The dispatch type for the Redux store.
 */
export type AppDispatch = typeof store.dispatch;
