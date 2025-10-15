import { configureStore } from "@reduxjs/toolkit";
import releasesReducer from "./releasesSlice";
import gameSessionReducer from "./gameSessionSlice";

export const store = configureStore({
  reducer: {
    releases: releasesReducer,
    gameSession: gameSessionReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
