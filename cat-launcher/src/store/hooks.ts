import { useDispatch, useSelector } from "react-redux";

import type { AppDispatch, RootState } from "./store";

/**
 * A custom hook that provides a typed `dispatch` function for the Redux store.
 */
export const useAppDispatch = useDispatch.withTypes<AppDispatch>();
/**
 * A custom hook that provides a typed `useSelector` function for the Redux store.
 */
export const useAppSelector = useSelector.withTypes<RootState>();
