import { useRef, useCallback, useEffect } from "react";

/**
 * A custom hook that returns a throttled version of the provided function, along with a cancel function.
 * The throttled function will only execute at most once every `delay` milliseconds.
 *
 * @param func - The function to throttle.
 * @param delay - The throttle delay in milliseconds.
 * @returns An object containing the throttled function and a cancel function to stop any pending executions.
 */
export function useThrottleWithCancel<Args extends unknown[]>(
  func: (...args: Args) => void,
  delay: number,
): {
  throttledFunc: (...args: Args) => void;
  cancel: () => void;
} {
  const lastCall = useRef(0);
  const timeoutId = useRef<ReturnType<typeof setTimeout> | null>(
    null,
  );
  const funcRef = useRef(func);
  funcRef.current = func;

  const throttledFunc = useCallback(
    (...args: Args): void => {
      const now = Date.now();
      const timeSinceLastCall = now - lastCall.current;

      if (timeSinceLastCall >= delay) {
        lastCall.current = now;
        funcRef.current(...args);
      } else {
        if (timeoutId.current) {
          clearTimeout(timeoutId.current);
        }
        timeoutId.current = setTimeout(() => {
          lastCall.current = Date.now();
          funcRef.current(...args);
        }, delay - timeSinceLastCall);
      }
    },
    [delay],
  );

  const cancel = useCallback(() => {
    if (timeoutId.current) {
      clearTimeout(timeoutId.current);
      timeoutId.current = null;
    }
  }, []);

  useEffect(() => {
    return () => {
      cancel();
    };
  }, [cancel]);

  return { throttledFunc, cancel };
}
