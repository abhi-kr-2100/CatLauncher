import { configureStore } from "@reduxjs/toolkit";

import gameSessionReducer from "./gameSessionSlice";
import installationProgressReducer from "./installationProgressSlice";
import releasesReducer from "./releasesSlice";

export const store = configureStore({
  reducer: {
    gameSession: gameSessionReducer,
    releases: releasesReducer,
    installationProgress: installationProgressReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
