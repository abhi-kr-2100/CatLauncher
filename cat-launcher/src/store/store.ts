import { configureStore } from "@reduxjs/toolkit";

import gameSessionReducer from "./gameSessionSlice";
import installationProgressReducer from "./installationProgressSlice";
import releasesReducer from "./releasesSlice";
import selectedVariantReducer from "./selectedVariantSlice";
import { settingsReducer } from "./settingsSlice";

export const store = configureStore({
  reducer: {
    gameSession: gameSessionReducer,
    releases: releasesReducer,
    installationProgress: installationProgressReducer,
    selectedVariant: selectedVariantReducer,
    settings: settingsReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
