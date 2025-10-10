import { configureStore } from "@reduxjs/toolkit";
import releasesReducer from "./releasesSlice";

export const store = configureStore({
  reducer: {
    releases: releasesReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
